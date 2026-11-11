#!/usr/bin/env python3
"""ZCode free-model gateway: zero paid-model fallback, live discovery, local OpenAI API.
Python 3.11+ standard library only. No secrets stored in this repository.
"""
from __future__ import annotations
import argparse, json, os, pathlib, threading, time, urllib.request, urllib.error, math
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from policy import strict_zero_price, nim_chat_candidate, demand, select, prepare, Quota, session_key, StickyRoutes, classify_upstream_error
import hmac
from kilo_gateway import discover as discover_kilo

BASE = pathlib.Path(os.environ.get("ZFREE_HOME", str(pathlib.Path.home() / ".zfree")))
OR_CATALOG = "https://openrouter.ai/api/v1/models"
OR_CHAT = "https://openrouter.ai/api/v1/chat/completions"
NIM_CATALOG = "https://integrate.api.nvidia.com/v1/models"
NIM_CHAT = "https://integrate.api.nvidia.com/v1/chat/completions"
CACHE_SECONDS = max(300, int(os.getenv("ZFREE_REFRESH_SECONDS", "3600")))
MAX_BODY = 8 * 1024 * 1024
STATE = {"at": 0.0, "models": [], "errors": []}
LOCK = threading.Lock()
# These limits are intentionally conservative. 50/day is the default OpenRouter
# allowance with no purchase history; owners may explicitly raise it to 1000.
QUOTA = Quota(BASE / "openrouter-quota.json", rpm=int(os.getenv("ZFREE_OR_RPM", "20")),
              daily=int(os.getenv("ZFREE_OR_DAILY_LIMIT", "50")))
KILO_QUOTA = Quota(BASE / "kilo-quota.json", rpm=int(os.getenv("ZFREE_KILO_RPM", "20")), daily=int(os.getenv("ZFREE_KILO_DAILY_LIMIT", "200")))
STICKY = StickyRoutes(ttl=21600)
COOLDOWN_LOCK = threading.Lock()
COOLDOWNS = {}

def cooling(model):
    with COOLDOWN_LOCK:
        return COOLDOWNS.get((model["provider"], model["id"]), 0) > time.time()

def pause_model(model, seconds):
    if seconds <= 0:
        return
    with COOLDOWN_LOCK:
        COOLDOWNS[(model["provider"], model["id"])] = time.time() + seconds

def request_json(url, key=None, timeout=20):
    headers = {"Accept": "application/json", "User-Agent": "zcode-free-model-toolkit/0.1"}
    if key:
        headers["Authorization"] = "Bearer " + key
    with urllib.request.urlopen(urllib.request.Request(url, headers=headers), timeout=timeout) as r:
        return json.load(r)

def is_zero(v):
    # A missing, invalid, negative, or non-finite price is not trusted.
    try:
        n = float(v)
        return math.isfinite(n) and n == 0.0
    except (TypeError, ValueError, OverflowError):
        return False

def openrouter_eligible(m):
    mid = m.get("id")
    pricing = m.get("pricing") or {}
    architecture = m.get("architecture") or {}
    output = architecture.get("output_modalities", ["text"])
    return (isinstance(mid, str) and mid.endswith(":free")
            and strict_zero_price(pricing)
            and isinstance(output, list) and "text" in output)

def normalized_request(body):
    """ZCode has shipped conflicting reasoning parameters for custom providers.
    Prefer a single broadly supported field; omit provider-specific aliases."""
    data = dict(body)
    data.pop("thinking", None)
    data.pop("enable_thinking", None)
    effort = data.get("reasoning_effort")
    if not isinstance(effort, str) and isinstance(data.get("reasoning"), dict):
        effort = data["reasoning"].get("effort")
    data.pop("reasoning", None)
    if isinstance(effort, str) and effort in ("minimal", "low", "medium", "high", "xhigh", "max"):
        data["reasoning_effort"] = effort
    else:
        data.pop("reasoning_effort", None)
    # Upstream is addressed with the selected actual model ID below.
    return data

def discover():
    """Never infer free from model name alone; classify by explicit billing evidence."""
    result, errors = [], []
    try:
        payload = request_json(OR_CATALOG)
        for m in payload.get("data", []):
            mid = m.get("id", "")
            pricing = m.get("pricing") or {}
            if not openrouter_eligible(m):
                continue
            params = m.get("supported_parameters") or []
            architecture = m.get("architecture") or {}
            modality = architecture.get("modality", "")
            top_provider = m.get("top_provider") or {}
            result.append({"provider": "openrouter", "id": mid, "name": m.get("name", mid),
                           "context_length": m.get("context_length", 0),
                           "max_completion_tokens": top_provider.get("max_completion_tokens"),
                           "input_modalities": architecture.get("input_modalities", ["text"]),
                           "supported_parameters": params if isinstance(params, list) else [],
                           "supported_efforts": (m.get("reasoning") or {}).get("supported_efforts"),
                           "modality": modality,
                           "endpoint": OR_CHAT, "evidence": "live-catalog-zero-price"})
    except Exception as e:
        errors.append("openrouter catalog: " + type(e).__name__ + ": " + str(e)[:160])
    # NVIDIA Developer Program credits are a free *prototype allowance*, not a
    # perpetual zero-dollar contract. Keep opt-in separated from guaranteed $0.
    if os.getenv("ZFREE_ALLOW_NIM_TRIAL", "0") == "1" and os.getenv("NVIDIA_API_KEY"):
        try:
            payload = request_json(NIM_CATALOG, os.getenv("NVIDIA_API_KEY"))
            for m in payload.get("data", []):
                mid = m.get("id", "")
                if isinstance(mid, str) and nim_chat_candidate(mid):
                    result.append({"provider": "nvidia-nim", "id": mid, "name": mid,
                                   "endpoint": NIM_CHAT,
                                   "evidence": "developer-trial-only-not-unconditionally-free"})
        except Exception as e:
            errors.append("nvidia catalog: " + type(e).__name__ + ": " + str(e)[:160])
    # OpenCode free chat models are explicitly OpenCode-client-only.
    # Do not route ZCode requests through OpenCode Console and pretend it is free.
    # Kilo Gateway is a separate upstream; never treat its catalogue as OpenRouter billing truth.
    if os.getenv("ZFREE_ALLOW_KILO", "1") == "1":
        try:
            result.extend(discover_kilo())
        except Exception as e:
            errors.append("kilo catalog: " + type(e).__name__ + ": " + str(e)[:160])
    result.sort(key=lambda x: (x["provider"], -int(x.get("context_length") or 0), x["id"]))
    return result, errors

def refresh(force=False):
    with LOCK:
        if not force and time.time() - STATE["at"] < CACHE_SECONDS:
            return STATE
        models, errors = discover()
        # Fail closed: never reclassify stale models or previous availability as free.
        STATE.update(at=time.time(), models=models, errors=errors)
        BASE.mkdir(parents=True, exist_ok=True)
        path = BASE / "catalog.json"
        tmp = path.with_suffix(".tmp")
        tmp.write_text(json.dumps({"updated_at": STATE["at"], "models": models,
                                  "errors": errors}, indent=2), encoding="utf-8")
        tmp.replace(path)
        return STATE

def eligible():
    st = refresh()
    allow_trial = os.getenv("ZFREE_ALLOW_NIM_TRIAL", "0") == "1"
    items = [m for m in st["models"] if
             (m["provider"] == "openrouter" and bool(os.getenv("OPENROUTER_API_KEY"))) or
             (m["provider"] == "nvidia-nim" and allow_trial and bool(os.getenv("NVIDIA_API_KEY"))) or
             (m["provider"] == "kilo-gateway" and m.get("routable") is True and os.getenv("ZFREE_ALLOW_KILO", "1") == "1")]
    # Prefer live documented $0 price over a trial quota. Custom preference
    # ordered by exact model IDs, never arbitrary user-supplied endpoint URLs.
    preferred = [v.strip() for v in os.getenv("ZFREE_PREFER", "").split(",") if v.strip()]
    return sorted(items, key=lambda m: (0 if m.get("route_id", m["id"]) in preferred or m["id"] in preferred else 1,
                                         preferred.index(m.get("route_id", m["id"])) if m.get("route_id", m["id"]) in preferred else
                                         preferred.index(m["id"]) if m["id"] in preferred else 0,
                                         {"openrouter": 0, "kilo-gateway": 1, "nvidia-nim": 2}.get(m["provider"], 3),
                                         -int(m.get("context_length") or 0), m["id"]))

class Handler(BaseHTTPRequestHandler):
    protocol_version = "HTTP/1.1"
    def log_message(self, fmt, *args):
        # Avoid leaking prompts and Authorization headers into logs.
        print("zfree request", self.command, self.path.split("?")[0], flush=True)

    def respond(self, status, payload):
        raw = json.dumps(payload).encode()
        self.send_response(status)
        self.send_header("Content-Type", "application/json")
        self.send_header("Content-Length", str(len(raw)))
        self.end_headers()
        self.wfile.write(raw)

    def authorized(self):
        key = os.getenv("ZFREE_LOCAL_TOKEN", "")
        supplied = self.headers.get("Authorization", "")
        return bool(key) and hmac.compare_digest(supplied, "Bearer " + key)

    def do_GET(self):
        if not self.authorized():
            return self.respond(401, {"error": {"message": "Unauthorized"}})
        if self.path == "/health":
            st = refresh()
            return self.respond(200, {"status": "ok" if eligible() else "no_eligible_models",
                                      "models": len(eligible()), "errors": st["errors"]})
        if self.path in ("/v1/models", "/models"):
            return self.respond(200, {"object": "list", "data": [
                {"id": "free-auto", "object": "model", "owned_by": "zfree"},
                *[{"id": m.get("route_id", m["id"]), "object": "model", "owned_by": m["provider"]}
                  for m in eligible()]]})
        return self.respond(404, {"error": {"message": "Unknown endpoint"}})

    def do_POST(self):
        if not self.authorized():
            return self.respond(401, {"error": {"message": "Unauthorized"}})
        if self.path != "/v1/chat/completions":
            return self.respond(404, {"error": {"message": "Only chat completions supported"}})
        try:
            length = int(self.headers.get("Content-Length", "0"))
            if length < 1 or length > MAX_BODY:
                return self.respond(413, {"error": {"message": "Invalid body size"}})
            body = json.loads(self.rfile.read(length))
            if not isinstance(body, dict):
                return self.respond(400, {"error": {"message": "Expected object"}})
        except (ValueError, json.JSONDecodeError):
            return self.respond(400, {"error": {"message": "Invalid JSON"}})
        try:
            demand(body)
        except ValueError as e:
            return self.respond(400, {"error": {"message": str(e)}})
        nim_tool = [v.strip() for v in os.getenv("ZFREE_NIM_TOOL_MODELS", "").split(",") if v.strip()]
        nim_vision = [v.strip() for v in os.getenv("ZFREE_NIM_VISION_MODELS", "").split(",") if v.strip()]
        candidate_pool = select(eligible(), body, nim_tool_ids=nim_tool, nim_vision_ids=nim_vision)
        candidates = [m for m in candidate_pool if not cooling(m)]
        if candidate_pool and not candidates:
            return self.respond(429, {"error": {"message": "All compatible free models temporarily cooling down; retry later"}})
        session_id = session_key(body, self.headers.get("X-ZFree-Session"))
        pinned = STICKY.get(session_id) if body.get("model", "free-auto") == "free-auto" else None
        if pinned:
            # Never silently swap models mid-session; thinking/tool histories may
            # be incompatible. New session/request identity can choose anew.
            candidates = [m for m in candidates if m.get("route_id", m["id"]) == pinned]
            if not candidates:
                return self.respond(409, {"error": {"message": "Pinned free model unavailable or lacks required capabilities; start a new session"}})
        if not candidates:
            return self.respond(503, {"error": {"message": "No currently eligible free model; refusing paid fallback"}})
        errors = []
        try:
            max_attempts = min(4, max(1, int(os.getenv("ZFREE_MAX_ATTEMPTS", "2"))))
        except ValueError:
            max_attempts = 4
        quota_wait = 0
        for m in candidates[:max_attempts]:
            provider_key = (os.getenv("OPENROUTER_API_KEY") if m["provider"] == "openrouter" else
                            os.getenv("KILO_API_KEY") if m["provider"] == "kilo-gateway" else
                            os.getenv("NVIDIA_API_KEY"))
            if m["provider"] == "openrouter":
                allowed, wait_seconds = QUOTA.take()
                if not allowed:
                    quota_wait = max(quota_wait, wait_seconds)
                    errors.append({"provider": "openrouter", "reason": "local_rate_limit"})
                    continue
            if m["provider"] == "kilo-gateway":
                allowed, wait_seconds = KILO_QUOTA.take()
                if not allowed:
                    quota_wait = max(quota_wait, wait_seconds)
                    errors.append({"provider": "kilo-gateway", "reason": "local_rate_limit"})
                    continue
            data = prepare(body, m)
            response_started = False
            headers = {"Content-Type": "application/json"}
            if provider_key:
                headers["Authorization"] = "Bearer " + provider_key
            if m["provider"] == "openrouter":
                headers["HTTP-Referer"] = "http://127.0.0.1"
                headers["X-Title"] = "ZCode Free Gateway"
            req = urllib.request.Request(m["endpoint"], data=json.dumps(data).encode(), headers=headers)
            try:
                with urllib.request.urlopen(req, timeout=90) as upstream:
                    if body.get("model", "free-auto") == "free-auto":
                        STICKY.put(session_id, m.get("route_id", m["id"]))
                    response_started = True
                    self.send_response(upstream.status)
                    self.send_header("Content-Type", upstream.headers.get("Content-Type", "application/json"))
                    self.send_header("Cache-Control", "no-cache")
                    self.send_header("Connection", "close")
                    self.end_headers()
                    # Raw streaming, including SSE; do not retry after bytes transmitted.
                    is_sse = "text/event-stream" in upstream.headers.get("Content-Type", "")
                    while True:
                        # Line-wise SSE avoids waiting for a full 8 KiB before first token.
                        chunk = upstream.readline() if is_sse else upstream.read(8192)
                        if not chunk:
                            break
                        self.wfile.write(chunk)
                        self.wfile.flush()
                    self.close_connection = True
                    return
            except urllib.error.HTTPError as e:
                if response_started:
                    self.close_connection = True
                    return
                errors.append({"model": m["id"], "status": e.code})
                detail = e.read(2048).decode("utf-8", errors="replace")
                retry, seconds = classify_upstream_error(e.code, detail)
                pause_model(m, seconds)
                if not retry:
                    break
            except (urllib.error.URLError, TimeoutError, OSError) as e:
                if response_started:
                    # Headers or data already sent: never append a second HTTP response.
                    self.close_connection = True
                    return
                errors.append({"model": m["id"], "error": type(e).__name__})
        if quota_wait and all(e.get("reason") == "local_rate_limit" for e in errors):
            return self.respond(429, {"error": {"message": "Local free-tier quota exceeded",
                                               "retry_after_seconds": quota_wait}})
        return self.respond(503, {"error": {"message": "All eligible free routes failed; no paid fallback",
                                            "attempts": errors}})

def main():
    ap = argparse.ArgumentParser(description="Local free-model gateway for ZCode")
    ap.add_argument("command", choices=["serve", "scan", "doctor"])
    ap.add_argument("--port", type=int, default=int(os.getenv("ZFREE_PORT", "8765")))
    args = ap.parse_args()
    if args.command in ("scan", "doctor"):
        st = refresh(True)
        print(json.dumps({"model_count": len(st["models"]), "eligible_count": len(eligible()),
                          "providers": sorted(set(m["provider"] for m in st["models"])),
                          "errors": st["errors"], "cache": str(BASE / "catalog.json"),
                          "opencode": "OpenCode free chat models not available to third-party ZCode",
                          "kilo": "Only catalog-verified :free or documented kilo-auto/free; anonymous access may be rate limited"},
                         indent=2))
        if args.command == "doctor":
            print("ZCode custom OpenAI Chat Completions provider: http://127.0.0.1:%d/v1 ; model: free-auto" % args.port)
        return
    if not os.getenv("ZFREE_LOCAL_TOKEN"):
        ap.error("Set ZFREE_LOCAL_TOKEN before serving; unauthenticated local proxies are unsafe")
    refresh(True)
    server = ThreadingHTTPServer(("127.0.0.1", args.port), Handler)
    print("ZFree listening on http://127.0.0.1:%d/v1" % args.port, flush=True)
    try:
        server.serve_forever(poll_interval=0.5)
    except KeyboardInterrupt:
        pass
    finally:
        server.server_close()

if __name__ == "__main__":
    main()
