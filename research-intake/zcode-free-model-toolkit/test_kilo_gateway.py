"""No-network tests for Kilo catalog validation and model identifiers."""
import unittest
from unittest.mock import patch
import kilo_gateway as kg


def model(mid, prompt='0', completion='0', **kwargs):
    return {'id': mid, 'name': mid, 'pricing': {'prompt': prompt, 'completion': completion}, **kwargs}


class KiloTests(unittest.TestCase):
    def test_only_explicit_zero_and_free_marker(self):
        result = kg.extract_models({'data': [
            model('z-ai/glm:free'), model('bad/paid', '0', '1'),
            model('bad/trap:free', '0', '0.001'), model('bad/zero-without-free'),
            {'id': 'bad/missing:free', 'pricing': {'prompt': '0'}},
            model('bad/addon:free', '0', '0', pricing_dummy='ignored'),
        ]})
        self.assertEqual([x['id'] for x in result], ['bad/addon:free', 'z-ai/glm:free'])

    def test_nonzero_auxiliary_price_denied(self):
        m = model('x/y:free'); m['pricing']['request'] = '0.2'
        self.assertEqual(kg.extract_models({'data': [m]}), [])

    def test_invalid_prices_denied(self):
        for x in ('nan', 'Infinity', 'abc', '-1', '', None):
            self.assertFalse(kg.confirmed_zero({'prompt': '0', 'completion': x}))

    def test_auto_free_requires_no_positive_prices(self):
        self.assertEqual(kg.free_classification({'id': 'kilo-auto/free'}), 'documented-auto-free-no-credit')
        self.assertIsNone(kg.free_classification(model('kilo-auto/free', '0', '2')))
        self.assertIsNone(kg.free_classification({'id': 'kilo-auto/frontier'}))
        self.assertIsNone(kg.free_classification({'id': 'kilo-auto/small'}))

    def test_alias_does_not_mutate_upstream_id(self):
        result = kg.extract_models({'data': [model('z-ai/glm:free')]})[0]
        self.assertEqual(result['id'], 'z-ai/glm:free')
        self.assertEqual(result['route_id'], 'kilo/z-ai/glm:free')
        self.assertEqual(result['endpoint'], kg.CHAT_URL)

    def test_no_tools_invented(self):
        result = kg.extract_models({'data': [model('z-ai/glm:free')]})[0]
        self.assertEqual(result['supported_parameters'], [])

    def test_text_only_and_duplicate_ids(self):
        records = [model('x/img:free', architecture={'output_modalities': ['image']}),
                   model('x/text:free'), model('x/text:free')]
        self.assertEqual(len(kg.extract_models({'data': records})), 1)

    def test_bad_catalog_fails_closed(self):
        for payload in ({}, {'data': None}, [], {'data': 'bogus'}):
            with self.assertRaises(ValueError):
                kg.extract_models(payload)

    def test_query_filters_and_limits(self):
        with patch.object(kg, 'fetch_catalog', return_value={'data': [model('x/alpha:free'), model('x/beta:free')]}):
            self.assertEqual([x['id'] for x in kg.query('ALPHA')], ['x/alpha:free'])
            self.assertEqual(len(kg.query(limit=1)), 1)

    def test_paid_catalog_only_for_read_only_inspection(self):
        out = kg.extract_models({'data': [model('p/model', '1', '1')]}, include_paid=True)
        self.assertFalse(out[0]['routable'])
        self.assertEqual(kg.extract_models({'data': [model('p/model', '1', '1')]}), [])

if __name__ == '__main__':
    unittest.main()