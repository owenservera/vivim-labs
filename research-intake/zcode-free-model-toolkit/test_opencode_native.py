"""Tests for safe read-only OpenCode inventory."""
import unittest
from opencode_native import extract, free_cost, inspect

class NativeTests(unittest.TestCase):
    def test_zero_cost(self):
        self.assertTrue(free_cost({"input": 0, "output": 0, "cache": {"read": 0, "write": 0}}))
        self.assertFalse(free_cost({"input": 0, "output": 0.01}))
        self.assertFalse(free_cost({"input": 0}))

    def test_provider_only(self):
        data = {"all": [
            {"id": "opencode", "models": {"good": {"cost": {"input": 0, "output": 0}},
                                             "paid": {"cost": {"input": 0.5, "output": 1}}}},
            {"id": "anthropic", "models": {"fake": {"cost": {"input": 0, "output": 0}}}}]}
        self.assertEqual([m["id"] for m in extract(data)], ["opencode/good"])

    def test_loopback_only(self):
        with self.assertRaises(ValueError):
            inspect("https://example.com")

if __name__ == "__main__":
    unittest.main()
