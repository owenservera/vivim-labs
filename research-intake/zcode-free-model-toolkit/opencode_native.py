#!/usr/bin/env python3
"""Read-only OpenCode native free-model inventory; never proxies restricted free tier to ZCode."""
import argparse
import base64
import json
import os
import urllib.parse
import urllib.request

def zero(v):
    return isinstance(v, (int, float)) and not isinstance(v, bool) and v == 0

def free_cost(cost):
    """Strictly require explicit zero input/output and all published cache prices."""
    if not isinstance(cost, dict) or not all(k in cost for k in ("input", "output")):
        return False
    if not zero(cost["input"]) or not zero(cost["output"]):
        return False
    cache = cost.get("cache", {}) or {}
    return isinstance(cache, dict) and all(zero(v) for v in cache.values())

def extract(payload):
    """GET /provider: {all:Provider[], connected:string[]}.
    Shape changes result in no eligible entries, not invented free models.
    """
    result = []
    providers = payload.get("all", []) if isinstance(payload, dict) else []
    for provider in providers:
        if not isinstance(provider, dict):
            continue
        pid = provider.get("id")
        if pid not in ("opencode", "opencode-go"):
            continue
        models = provider.get("models") or {}
        if not isinstance(models, dict):
            continue
        for mid, record in models.items():
            if not isinstance(record, dict):
                continue
            if free_cost(record.get("cost")):
                result.append({"id": f"{pid}/{mid}", "name": record.get("name", mid),
                               "free_evidence": "local-opencode-provider-zero-cost",
                               "zcode_routable": False})
    return sorted(result, key=lambda x: x["id"])

def inspect(url, password=None):
    parsed = urllib.parse.urlparse(url)
    if parsed.scheme != "http" or parsed.hostname not in ("127.0.0.1", "localhost", "::1"):
        raise ValueError("Only loopback HTTP OpenCode servers are allowed")
    request = urllib.request.Request(url.rstrip("/") + "/provider", headers={"Accept": "application/json"})
    if password:
        creds = base64.b64encode(("opencode:" + password).encode()).decode()
        request.add_header("Authorization", "Basic " + creds)
    with urllib.request.urlopen(request, timeout=6) as response:
        return extract(json.load(response))

def main():
    p = argparse.ArgumentParser(description="Read-only OpenCode native free model report")
    p.add_argument("--url", default=os.getenv("ZFREE_OPENCODE_URL", "http://127.0.0.1:4096"))
    args = p.parse_args()
    try:
        models = inspect(args.url, os.getenv("OPENCODE_SERVER_PASSWORD"))
        print(json.dumps({"status": "connected", "native_free_models": models,
                          "zcode_routing": "not-allowed-by-opencode-free-tier"}, indent=2))
    except Exception as e:
        print(json.dumps({"status": "unavailable", "native_free_models": [],
                          "reason": type(e).__name__,
                          "hint": "Start 'opencode serve --port 4096' locally, then rerun; no model use performed"}, indent=2))
        raise SystemExit(1)

if __name__ == "__main__":
    main()
