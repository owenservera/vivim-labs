"""Offline regression checks for ZFree. Run: python -m unittest discover -s . -p 'test_*.py'"""
import json
import os
import unittest
from unittest import mock
import zfree

class DiscoveryTests(unittest.TestCase):
    def setUp(self):
        zfree.STATE.update(at=0, models=[], errors=[])
    def test_openrouter_requires_free_suffix_and_both_zero_prices(self):
        fake = {"data": [
            {"id": "good:free", "pricing": {"prompt": "0", "completion": "0"}, "context_length": 8192},
            {"id": "paid", "pricing": {"prompt": "0", "completion": "0"}},
            {"id": "trap:free", "pricing": {"prompt": "0", "completion": "0.001"}},
            {"id": "unknown:free", "pricing": {"prompt": "0"}},
        ]}
        with mock.patch.object(zfree, "request_json", return_value=fake), \
             mock.patch.dict(os.environ, {"ZFREE_ALLOW_NIM_TRIAL": "0"}, clear=True):
            models, errs = zfree.discover()
        self.assertFalse(errs)
        self.assertEqual([m["id"] for m in models], ["good:free"])

    def test_nim_is_opt_in(self):
        fake = {"data": [{"id": "nemotron", "pricing": {}}]}
        with mock.patch.object(zfree, "request_json", return_value=fake), \
             mock.patch.dict(os.environ, {"NVIDIA_API_KEY": "dummy", "ZFREE_ALLOW_NIM_TRIAL": "0"}, clear=True):
            models, _ = zfree.discover()
        self.assertEqual(models, [])

    def test_nim_trial_is_distinct_from_zero_price(self):
        def reply(url, key=None, timeout=20):
            if "nvidia" in url:
                return {"data": [{"id": "nv/model"}]}
            return {"data": []}
        with mock.patch.object(zfree, "request_json", side_effect=reply), \
             mock.patch.dict(os.environ, {"NVIDIA_API_KEY": "dummy", "ZFREE_ALLOW_NIM_TRIAL": "1"}, clear=True):
            models, _ = zfree.discover()
        self.assertEqual(models[0]["evidence"], "developer-trial-only-not-unconditionally-free")

    def test_no_api_keys_means_no_eligible_routes(self):
        with mock.patch.object(zfree, "refresh", return_value=None), \
             mock.patch.dict(os.environ, {}, clear=True):
            zfree.STATE["models"] = [{"provider": "openrouter", "id": "model:free", "context_length": 100}]
            self.assertEqual(zfree.eligible(), [])

    def test_auxiliary_request_price_is_not_free(self):
        model = {"id": "trap:free", "pricing":
                 {"prompt": "0", "completion": "0", "request": "0.02"}}
        self.assertFalse(zfree.openrouter_eligible(model))

    def test_requires_text_output(self):
        model = {"id": "image:free", "pricing": {"prompt": "0", "completion": "0"},
                 "architecture": {"output_modalities": ["image"]}}
        self.assertFalse(zfree.openrouter_eligible(model))

    def test_reasoning_collision_normalized(self):
        payload = {"model": "free-auto", "messages": [{"role": "user", "content": "hi"}],
                   "thinking": {"type": "enabled"}, "enable_thinking": True,
                   "reasoning": {"effort": "high"}, "reasoning_effort": "medium"}
        cleaned = zfree.normalized_request(payload)
        self.assertEqual(cleaned["reasoning_effort"], "medium")
        self.assertNotIn("reasoning", cleaned)
        self.assertNotIn("thinking", cleaned)
        self.assertNotIn("enable_thinking", cleaned)

    def test_invalid_reasoning_omitted(self):
        self.assertNotIn("reasoning_effort", zfree.normalized_request(
            {"reasoning": {"effort": "provider-custom-unsupported"}}))

    def test_paid_model_never_eligible_even_if_preferred(self):
        with mock.patch.object(zfree, "refresh", return_value=None), \
             mock.patch.dict(os.environ, {"OPENROUTER_API_KEY": "dummy", "ZFREE_PREFER": "paid"}, clear=True):
            zfree.STATE["models"] = [{"provider": "openrouter", "id": "okay:free", "context_length": 100}]
            self.assertEqual([x["id"] for x in zfree.eligible()], ["okay:free"])

if __name__ == "__main__":
    unittest.main()
