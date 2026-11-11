"""Headless GUI view-model tests; no display or network required."""
import unittest
from model_explorer import filtered, normalize_models

class ExplorerModelTests(unittest.TestCase):
    def test_keeps_upstream_and_zcode_alias_distinct(self):
        rows = normalize_models([{'provider':'kilo-gateway','id':'x/y:free','route_id':'kilo/x/y:free'}])
        self.assertEqual(rows[0]['id'], 'x/y:free')
        self.assertEqual(rows[0]['route_id'], 'kilo/x/y:free')

    def test_opencode_cannot_route(self):
        rows = normalize_models([], [{'id': 'opencode/zero', 'name':'Zero'}])
        self.assertIsNone(rows[0]['route_id'])
        self.assertFalse(rows[0]['routable'])

    def test_search_and_provider_filter(self):
        rows = normalize_models([{'provider':'kilo-gateway','id':'x/alpha:free'},
                                 {'provider':'openrouter','id':'y/beta:free'}])
        self.assertEqual(len(filtered(rows, 'alpha')), 1)
        self.assertEqual(len(filtered(rows, provider='openrouter')), 1)

if __name__ == '__main__': unittest.main()