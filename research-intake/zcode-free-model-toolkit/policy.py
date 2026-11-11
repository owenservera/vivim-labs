"""ZFree routing safeguards. Pure standard-library module, no network or secrets."""
from __future__ import annotations
import json
import math
import pathlib
import threading
import time
from collections import deque

COST_FIELDS = frozenset({"plugins", "web_search_options", "web_search", "web_search_requests", "transforms"})
OPTIONAL_PARAMS = frozenset({"temperature", "top_p", "stop", "seed", "frequency_penalty", "presence_penalty", "logit_bias"})
NIM_DENY_TOKENS = ("embed", "rerank", "embedding", "retrieval", "nemoguard", "guard", "safety", "clip", "bge", "gliner", "riva", "translate", "reward", "classification", "text-to-image", "imagegen")

def strict_zero_price(pricing):
    if not isinstance(pricing, dict) or not pricing:
        return False
    if any(k not in pricing for k in ("prompt", "completion")):
        return False
    try:
        values = [float(v) for v in pricing.values()]
        return all(math.isfinite(n) and n == 0 for n in values)
    except (TypeError, ValueError, OverflowError):
        return False

def nim_chat_candidate(model_id):
    """Best-effort exclusions, not a guarantee of capability."""
    m = model_id.lower()
    return bool(m) and not any(x in m for x in NIM_DENY_TOKENS)

def contains_image(messages):
    for message in messages:
        if not isinstance(message, dict):
            continue
        content = message.get("content")
        if isinstance(content, list):
            for part in content:
                if isinstance(part, dict) and part.get("type") in ("image_url", "input_image", "image"):
                    return True
    return False

def demand(body):
    if not isinstance(body, dict):
        raise ValueError("Chat request must be an object")
    messages = body.get("messages")
    if not isinstance(messages, list) or not messages or any(not isinstance(x, dict) for x in messages):
        raise ValueError("messages must be a nonempty array of objects")
    if any(k in body for k in COST_FIELDS):
        raise ValueError("Paid or unknown add-on request parameter prohibited")
    tools = body.get("tools")
    if tools is not None and not isinstance(tools, list):
        raise ValueError("tools must be an array")
    if tools:
        for t in tools:
            if not isinstance(t, dict) or t.get("type") != "function":
                raise ValueError("Only client-executed function tools are allowed")
    model = body.get("model", "free-auto")
    if not isinstance(model, str) or not model:
        raise ValueError("model must be a nonempty string")
    if body.get("stream") is not None and not isinstance(body["stream"], bool):
        raise ValueError("stream must be boolean")
    return {"tools": bool(tools) or has_tool_history(body), "image": contains_image(messages),
            "strict_format": isinstance(body.get("response_format"), dict),
            "choice": body.get("tool_choice"), "model": model}

def select(models, body, nim_tool_ids=(), nim_vision_ids=()):
    req = demand(body)
    approved_tools = set(nim_tool_ids)
    approved_vision = set(nim_vision_ids)
    selected = []
    for m in models:
        if req["model"] != "free-auto" and m.get("route_id", m.get("id")) != req["model"]:
            continue
        provider = m.get("provider")
        # Kilo auto/free can switch underlying models: refuse stateful tool/image/JSON agent use.
        if provider == "kilo-gateway" and m.get("id") == "kilo-auto/free" and (req["tools"] or req["image"] or req["strict_format"]):
            continue
        params = set(m.get("supported_parameters") or ())
        inputs = m.get("input_modalities") or (["text"] if provider == "openrouter" else [])
        if req["tools"] and not ("tools" in params if provider in ("openrouter", "kilo-gateway") else m.get("id") in approved_tools):
            continue
        if req["image"] and not ("image" in inputs if provider in ("openrouter", "kilo-gateway") else m.get("id") in approved_vision):
            continue
        if req["strict_format"] and provider in ("openrouter", "kilo-gateway") and not ({"response_format", "structured_outputs"} & params):
            continue
        if req["strict_format"] and provider not in ("openrouter", "kilo-gateway"):
            continue
        choice = req["choice"]
        if choice not in (None, "auto", "none") and provider in ("openrouter", "kilo-gateway") and "tool_choice" not in params:
            continue
        selected.append(m)
    return selected

def prepare(body, model):
    """Normalize ZCode aliases without losing history/tool results."""
    data = dict(body)
    for k in ("thinking", "enable_thinking"):
        data.pop(k, None)
    effort = data.pop("reasoning_effort", None)
    nested = data.pop("reasoning", None)
    if effort is None and isinstance(nested, dict):
        effort = nested.get("effort")
    if effort not in ("none", "minimal", "low", "medium", "high", "xhigh", "max"):
        effort = None
    provider = model.get("provider")
    params = set(model.get("supported_parameters") or ())
    if provider == "openrouter":
        if effort not in (None, "none") and "reasoning" in params:
            supported = model.get("supported_efforts")
            if supported is None or effort in supported:
                data["reasoning"] = {"effort": effort}
        if "tool_choice" not in params and data.get("tool_choice") in ("auto", "none"):
            data.pop("tool_choice", None)
        if "parallel_tool_calls" not in params:
            data.pop("parallel_tool_calls", None)
        for k in OPTIONAL_PARAMS:
            if k in data and k not in params:
                data.pop(k, None)
    if provider == "kilo-gateway":
        # Only pass model-advertised options; no speculative reasoning translation.
        if "tool_choice" not in params and data.get("tool_choice") in ("auto", "none"):
            data.pop("tool_choice", None)
        if "parallel_tool_calls" not in params:
            data.pop("parallel_tool_calls", None)
        for k in OPTIONAL_PARAMS:
            if k in data and k not in params:
                data.pop(k, None)
        requested_limit = data.pop("max_completion_tokens", data.pop("max_tokens", None))
        if isinstance(requested_limit, int) and not isinstance(requested_limit, bool) and requested_limit > 0:
            field = ("max_completion_tokens" if "max_completion_tokens" in params else
                     "max_tokens" if "max_tokens" in params else None)
            if field:
                data[field] = requested_limit
    # Some free models advertise only max_tokens. Never send both aliases.
    if provider == "openrouter":
        limit = data.pop("max_completion_tokens", None)
        legacy = data.pop("max_tokens", None)
        if limit is None:
            limit = legacy
        if isinstance(limit, int) and not isinstance(limit, bool) and limit > 0:
            field = "max_completion_tokens" if "max_completion_tokens" in params else "max_tokens"
            data[field] = limit
    # NIM models vary. No speculative reasoning parameter translation.
    data["model"] = model["id"]
    max_allowed = model.get("max_completion_tokens")
    if isinstance(max_allowed, int) and max_allowed > 0:
        for k in ("max_completion_tokens", "max_tokens"):
            if isinstance(data.get(k), int) and data[k] > max_allowed:
                data[k] = max_allowed
    return data

class Quota:
    """Conservative OpenRouter cap including failures; persists across restarts."""
    def __init__(self, path, rpm=20, daily=50, clock=time.time):
        self.path = pathlib.Path(path)
        self.rpm = min(20, max(1, int(rpm)))
        self.daily = min(1000, max(1, int(daily)))
        self.clock = clock
        self.lock = threading.Lock()
        self.times = deque()
        self.corrupt = False
        if self.path.exists():
            try:
                payload = json.loads(self.path.read_text(encoding="utf-8"))
                if not isinstance(payload, list) or any(not isinstance(x, (int, float)) for x in payload):
                    raise ValueError("invalid local quota ledger")
                self.times.extend(payload)
            except (OSError, ValueError):
                self.corrupt = True

    def take(self):
        with self.lock:
            if self.corrupt:
                return False, 86400
            now = self.clock()
            while self.times and self.times[0] < now - 86400:
                self.times.popleft()
            recent = sum(x >= now - 60 for x in self.times)
            if len(self.times) >= self.daily:
                return False, max(1, int(self.times[0] + 86400 - now))
            if recent >= self.rpm:
                first = next(x for x in self.times if x >= now - 60)
                return False, max(1, int(first + 60 - now))
            self.times.append(now)
            self.path.parent.mkdir(parents=True, exist_ok=True)
            tmp = self.path.with_suffix(".tmp")
            try:
                tmp.write_text(json.dumps(list(self.times)), encoding="utf-8")
                tmp.replace(self.path)
            except OSError:
                self.times.pop()
                return False, 60
            return True, 0


def session_key(body, explicit=None):
    """Ephemeral best-effort identity. No prompts saved to disk or logs."""
    import hashlib
    if explicit and isinstance(explicit, str) and len(explicit) <= 256:
        basis = explicit
    else:
        head = []
        for item in body.get("messages", []):
            if item.get("role") in ("system", "developer", "user"):
                head.append([item.get("role"), str(item.get("content", ""))[:1024]])
            if len(head) >= 3:
                break
        basis = json.dumps(head, sort_keys=True)
    return hashlib.sha256(basis.encode("utf-8", errors="replace")).hexdigest()

def has_tool_history(body):
    return any(x.get("role") == "tool" or bool(x.get("tool_calls")) for x in body.get("messages", []))

class StickyRoutes:
    def __init__(self, ttl=21600, clock=time.time):
        self.lock = threading.Lock()
        self.ttl = ttl
        self.clock = clock
        self.rows = {}

    def get(self, key):
        with self.lock:
            entry = self.rows.get(key)
            if not entry:
                return None
            if entry[1] <= self.clock():
                self.rows.pop(key, None)
                return None
            return entry[0]

    def put(self, key, model_id):
        with self.lock:
            self.rows[key] = (model_id, self.clock() + self.ttl)
            if len(self.rows) > 4096:
                now = self.clock()
                self.rows = {k:v for k,v in self.rows.items() if v[1] > now}


def classify_upstream_error(status, details=""):
    """Returns (may_try_another_free_model, cooldown_seconds).
    Never retry authentication, billing, or unknown schema failures blindly.
    """
    details = str(details).lower()[:2048]
    if status == 429:
        if any(term in details for term in ("daily quota", "daily limit", "account limit")):
            return False, 3600
        return True, 60
    if status in (408, 500, 502, 503, 504):
        return True, 30
    if status == 403 and any(term in details for term in ("region", "country", "geograph", "geo-restrict")):
        return True, 3600
    if status == 404 and any(term in details for term in ("no endpoints", "tool use", "tools not supported")):
        return True, 1800
    return False, 0
