"""Kilo integration checks; all catalogue and HTTP activity mocked offline."""
import os
import unittest
from unittest import mock
import zfree
import policy

GOOD = {"provider": "kilo-gateway", "id": "z-ai/test:free",
        "route_id": "kilo/z-ai/test:free", "routable": True,
        "supported_parameters": ["max_tokens"], "input_modalities": ["text"],
        "context_length": 8192, "endpoint": "https://ai-gateway.kilo.ai/api/v1/chat/completions"}

class KiloIntegrationTests(unittest.TestCase):
    def setUp(self):
        zfree.STATE.update(at=0, models=[], errors=[])

    def test_discovery_merges_kilo_without_key(self):
        with mock.patch.object(zfree, "request_json", return_value={"data": []}), \
             mock.patch.object(zfree, "discover_kilo", return_value=[GOOD]), \
             mock.patch.dict(os.environ, {"ZFREE_ALLOW_KILO": "1"}, clear=True):
            models, errors = zfree.discover()
        self.assertEqual(errors, [])
        self.assertEqual(models, [GOOD])

    def test_kilo_free_without_key_but_never_unverified(self):
        with mock.patch.object(zfree, "refresh", return_value=zfree.STATE), \
             mock.patch.dict(os.environ, {"ZFREE_ALLOW_KILO": "1"}, clear=True):
            zfree.STATE["models"] = [GOOD, {**GOOD, "id": "paid", "routable": False}]
            rows = zfree.eligible()
        self.assertEqual([x["id"] for x in rows], [GOOD["id"]])

    def test_kilo_route_name_prevents_cross_provider_collision(self):
        body = {"model": GOOD["route_id"], "messages": [{"role": "user", "content": "hello"}]}
        openrouter = {**GOOD, "provider": "openrouter", "route_id": "z-ai/test:free"}
        self.assertEqual(policy.select([GOOD, openrouter], body), [GOOD])
        body["model"] = GOOD["id"]
        self.assertEqual(policy.select([GOOD, openrouter], body), [openrouter])

    def test_tools_not_assumed_from_openai_compatible_transport(self):
        body = {"model": GOOD["route_id"], "messages": [{"role": "user", "content": "hi"}],
                "tools": [{"type": "function", "function": {"name": "fn", "parameters": {}}}]}
        self.assertEqual(policy.select([GOOD], body), [])
        self.assertEqual(policy.select([{**GOOD, "supported_parameters": ["tools"]}], body)[0]["id"], GOOD["id"])

    def test_no_paid_fallback_even_with_explicit_preference(self):
        with mock.patch.object(zfree, "refresh", return_value=zfree.STATE), \
             mock.patch.dict(os.environ, {"ZFREE_PREFER": "paid", "ZFREE_ALLOW_KILO": "1"}, clear=True):
            zfree.STATE["models"] = [{**GOOD, "id": "paid", "routable": False}]
            self.assertEqual(zfree.eligible(), [])

    def test_kilo_prepare_uses_true_upstream_id_and_no_reasoning_guess(self):
        body = {"model": GOOD["route_id"], "messages": [{"role": "user", "content": "hi"}],
                "reasoning_effort": "high", "max_completion_tokens": 123}
        prepared = policy.prepare(body, GOOD)
        self.assertEqual(prepared["model"], GOOD["id"])
        self.assertEqual(prepared["max_tokens"], 123)
        self.assertNotIn("reasoning_effort", prepared)

if __name__ == "__main__":
    unittest.main()
