"""Loopback HTTP gateway acceptance tests. All upstream traffic is local simulated HTTP."""
import json
import os
import tempfile
import threading
import time
import unittest
import urllib.error
import urllib.request
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
import zfree
from policy import Quota, StickyRoutes

class FakeProvider(BaseHTTPRequestHandler):
    last_body = None
    status = 200

    def log_message(self, *args):
        pass

    def do_POST(self):
        size = int(self.headers.get("Content-Length", "0"))
        FakeProvider.last_body = json.loads(self.rfile.read(size))
        if FakeProvider.status != 200:
            self.send_response(FakeProvider.status)
            self.send_header("Content-Length", "0")
            self.end_headers()
            return
        if FakeProvider.last_body.get("stream"):
            raw = b'data: {"id":"c","choices":[{"delta":{"content":"hi"}}]}\n\ndata: [DONE]\n\n'
            ctype = "text/event-stream"
        else:
            raw = b'{"id":"c","choices":[{"message":{"role":"assistant","content":"hi"}}]}'
            ctype = "application/json"
        self.send_response(200)
        self.send_header("Content-Type", ctype)
        self.send_header("Content-Length", str(len(raw)))
        self.end_headers()
        self.wfile.write(raw)

def start(cls):
    server = ThreadingHTTPServer(("127.0.0.1", 0), cls)
    thread = threading.Thread(target=server.serve_forever, daemon=True)
    thread.start()
    return server, thread

class HttpTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.original_env = dict(os.environ)
        os.environ["OPENROUTER_API_KEY"] = "fake-key"
        os.environ["ZFREE_LOCAL_TOKEN"] = "test-local-secret"
        os.environ.pop("NVIDIA_API_KEY", None)
        self.upstream, self.upstream_thread = start(FakeProvider)
        FakeProvider.status = 200
        zfree.STATE.update(at=time.time(), errors=[], models=[{
            "id": "test/tools:free", "provider": "openrouter",
            "endpoint": f"http://127.0.0.1:{self.upstream.server_port}/v1/chat/completions",
            "context_length": 100000, "max_completion_tokens": 1024,
            "supported_parameters": ["tools", "reasoning", "tool_choice"],
            "input_modalities": ["text"], "supported_efforts": ["high"]}])
        zfree.QUOTA = Quota(os.path.join(self.temp.name, "quota.json"), rpm=20, daily=50)
        zfree.STICKY = StickyRoutes()
        self.server, self.thread = start(zfree.Handler)
        self.url = f"http://127.0.0.1:{self.server.server_port}/v1/chat/completions"

    def tearDown(self):
        self.server.shutdown()
        self.server.server_close()
        self.upstream.shutdown()
        self.upstream.server_close()
        os.environ.clear()
        os.environ.update(self.original_env)
        self.temp.cleanup()

    def post(self, body, token=True):
        request = urllib.request.Request(
            self.url, method="POST", data=json.dumps(body).encode(),
            headers={"Content-Type": "application/json",
                     "Authorization": "Bearer test-local-secret" if token else "Bearer wrong"})
        try:
            with urllib.request.urlopen(request, timeout=5) as response:
                return response.status, response.read().decode()
        except urllib.error.HTTPError as error:
            return error.code, error.read().decode()

    def test_local_auth_required(self):
        status, _ = self.post({"messages": [{"role": "user", "content": "x"}]}, False)
        self.assertEqual(status, 401)

    def test_plugin_upsell_rejected(self):
        status, _ = self.post({"messages": [{"role": "user", "content": "x"}], "plugins": [{"id": "web"}]})
        self.assertEqual(status, 400)

    def test_paid_model_rejected(self):
        status, _ = self.post({"model": "paid/model", "messages": [{"role": "user", "content": "x"}]})
        self.assertEqual(status, 503)

    def test_json_tool_history_and_schema(self):
        payload = {"messages": [
            {"role": "system", "content": "sys"},
            {"role": "user", "content": "x"},
            {"role": "tool", "tool_call_id": "abc", "content": "{}"}],
            "tools": [{"type": "function", "function": {"name": "read"}}],
            "reasoning": {"effort": "high"}, "thinking": {"type": "enabled"},
            "max_completion_tokens": 9000}
        status, raw = self.post(payload)
        self.assertEqual(status, 200, raw)
        self.assertEqual(FakeProvider.last_body["model"], "test/tools:free")
        self.assertEqual(FakeProvider.last_body["reasoning"], {"effort": "high"})
        self.assertNotIn("thinking", FakeProvider.last_body)
        self.assertEqual(FakeProvider.last_body["max_completion_tokens"], 1024)
        self.assertEqual(FakeProvider.last_body["messages"], payload["messages"])

    def test_sse_passthrough(self):
        status, raw = self.post({"messages": [{"role": "user", "content": "stream"}], "stream": True})
        self.assertEqual(status, 200, raw)
        self.assertIn("data: [DONE]", raw)

    def test_quota_includes_failures(self):
        zfree.QUOTA = Quota(os.path.join(self.temp.name, "newquota.json"), rpm=20, daily=1)
        FakeProvider.status = 429
        status, _ = self.post({"messages": [{"role": "user", "content": "try"}]})
        self.assertEqual(status, 503)
        status, _ = self.post({"messages": [{"role": "user", "content": "try"}]})
        self.assertEqual(status, 429)

if __name__ == "__main__":
    unittest.main()
