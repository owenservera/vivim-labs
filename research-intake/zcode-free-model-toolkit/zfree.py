#!/usr/bin/env python3
"""ZCode free-model gateway: zero paid-model fallback, live discovery, local OpenAI API.
Python 3.11+ standard library only. No secrets stored in this repository.
"""
from __future__ import annotations
import argparse, json, os, pathlib, threading, time, urllib.request, urllib.error
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer

BASE = pathlib.Path(os.environ.get("ZFREE_HOME", str(pathlib.Path.home() / ".zfree")))
OR_CATALOG = "https://openrouter.ai/api/v1/models"
OR_CHAT = "https://openrouter.ai/api/v1/chat/completions"
NIM_CATALOG = "https://integrate.api.nvidia.com/v1/models"
NIM_CHAT = "https://integrate.api.nvidia.com/v1/chat/completions"
CACHE_SECONDS = max(300, int(os.getenv("ZFREE_REFRESH_SECONDS", "3600")))
MAX_BODY = 8 * 1024 * 1024
STATE = {"at": 0.0, "models": [], "errors": []}
LOCK = threading.Lock()

def request_json(url, key=None, timeout=20):
    headers = {"Accept": "application/json", "User-Agent": "zcode-free-model-toolkit/0.1"}
    if key:
        headers["Authorization"] = "Bearer " + key
    with urllib.request.urlopen(urllib.request.Request(url, headers=headers), timeout=timeout) as r:
        return json.load(r)

def is_zero(v):
    try:
        return float(v) == 0.0
    except (TypeError, ValueError, OverflowError):
        return False

def discover():
    """Never infer free from model name alone; classify by explicit billing evidence."""
    result, errors = [], []
    try:
        payload = request_json(OR_CATALOG)
        for m in payload.get("data", []):
            mid = m.get("id", "")
            pricing = m.get("pricing") or {}
            if not isinstance(mid, str) or not mid.endswith(":free"):
                continue
            if not all(is_zero(pricing.get(k)) for k in ("prompt", "completion")):
                continue
            result.append({"provider": "openrouter", "id": mid, "name": m.get("name", mid),
                           "context_length": m.get("context_length", 0),
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
                if isinstance(mid, str) and mid:
                    result.append({"provider": "nvidia-nim", "id": mid, "name": mid,
                                   "endpoint": NIM_CHAT,
                                   "evidence": "developer-trial-only-not-unconditionally-free"})
        except Exception as e:
            errors.append("nvidia catalog: " + type(e).__name__ + ": " + str(e)[:160])
    # OpenCode free chat models are explicitly OpenCode-client-only.
    # Do not route ZCode requests through OpenCode Console and pretend it is free.
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
             (m["provider"] == "nvidia-nim" and allow_trial and bool(os.getenv("NVIDIA_API_KEY")))]
    # Prefer live documented $0 price over a trial quota. Custom preference
    # ordered by exact model IDs, never arbitrary user-supplied endpoint URLs.
    preferred = [v.strip() for v in os.getenv("ZFREE_PREFER", "").split(",") if v.strip()]
    return sorted(items, key=lambda m: (0 if m["id"] in preferred else 1,
                                         preferred.index(m["id"]) if m["id"] in preferred else 0,
                                         0 if m["provider"] == "openrouter" else 1,
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
        key = os.getenv("ZFREE_LOCAL_TOKEN")
        return not key or self.headers.get("Authorization") == "Bearer " + key

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
                *[{"id": m["id"], "object": "model", "owned_by": m["provider"]}
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
        candidates = eligible()
        wanted = body.get("model", "free-auto")
        if wanted != "free-auto":
            candidates = [m for m in candidates if m["id"] == wanted]
        if not candidates:
            return self.respond(503, {"error": {"message": "No currently eligible free model; refusing paid fallback"}})
        errors = []
        for m in candidates[:max(1, int(os.getenv("ZFREE_MAX_ATTEMPTS", "4")))]:
            key = os.getenv("OPENROUTER_API_KEY") if m["provider"] == "openrouter" else os.getenv("NVIDIA_API_KEY")
            data = dict(body, model=m["id"])
            headers = {"Authorization": "Bearer " + key, "Content-Type": "application/json"}
            if m["provider"] == "openrouter":
                headers["HTTP-Referer"] = "http://127.0.0.1"
                headers["X-Title"] = "ZCode Free Gateway"
            req = urllib.request.Request(m["endpoint"], data=json.dumps(data).encode(), headers=headers)
            try:
                with urllib.request.urlopen(req, timeout=90) as upstream:
                    self.send_response(upstream.status)
                    self.send_header("Content-Type", upstream.headers.get("Content-Type", "application/json"))
                    self.send_header("Cache-Control", "no-cache")
                    self.send_header("Connection", "close")
                    self.end_headers()
                    # Raw streaming, including SSE; do not retry after bytes transmitted.
                    while True:
                        chunk = upstream.read(8192)
                        if not chunk:
                            break
                        self.wfile.write(chunk)
                        self.wfile.flush()
                    self.close_connection = True
                    return
            except urllib.error.HTTPError as e:
                errors.append({"model": m["id"], "status": e.code})
                # Retry only non-billing failures. Never switch to paid models.
                if e.code not in (408, 429, 500, 502, 503, 504):
                    break
            except (urllib.error.URLError, TimeoutError, OSError) as e:
                errors.append({"model": m["id"], "error": type(e).__name__})
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
                          "opencode": "OpenCode free chat models not available to third-party ZCode"},
                         indent=2))
        if args.command == "doctor":
            print("ZCode custom OpenAI Chat Completions provider: http://127.0.0.1:%d/v1 ; model: free-auto" % args.port)
        return
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
