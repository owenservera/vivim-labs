#!/usr/bin/env python3
"""Kilo Gateway read-only model query and strictly-free routing eligibility.

No provider secret is stored; the catalogue URL is pinned to Kilo's documented
public OpenAI-compatible models endpoint. Python 3.11+, standard library only.
"""
from __future__ import annotations
import argparse
import json
import math
import os
import re
import sys
import urllib.request

CATALOG_URL = 'https://ai-gateway.kilo.ai/api/v1/models'
CHAT_URL = 'https://ai-gateway.kilo.ai/api/v1/chat/completions'
DOCS_URL = 'https://kilo.ai/docs/gateway/models-and-providers'


def confirmed_zero(pricing):
    """Reject missing, unparseable, negative, or non-zero published price fields."""
    if not isinstance(pricing, dict) or not pricing or not {'prompt', 'completion'} <= pricing.keys():
        return False
    try:
        nums = [float(v) for v in pricing.values()]
    except (TypeError, ValueError, OverflowError):
        return False
    return bool(nums) and all(math.isfinite(v) and v == 0 for v in nums)


def free_classification(record):
    """Require BOTH free designation and price evidence, except documented Auto Free.

    Auto Free has Kilo's explicit no-credits-required guarantee. A positive,
    missing-subfield or invalid *published* price is never ignored.
    """
    if not isinstance(record, dict) or not isinstance(record.get('id'), str):
        return None
    mid = record['id']
    price = record.get('pricing')
    if mid == 'kilo-auto/free':
        if price is None or price == {}:
            return 'documented-auto-free-no-credit'
        return 'catalog-zero-and-documented-auto-free' if confirmed_zero(price) else None
    if mid.endswith(':free') and confirmed_zero(price):
        return 'catalog-zero-free-id'
    return None


def extract_models(payload, *, include_paid=False):
    """Normalize model inventory while preserving exact upstream model IDs."""
    entries = payload.get('data') if isinstance(payload, dict) else None
    if not isinstance(entries, list):
        raise ValueError('Kilo catalog missing data[]')
    result = []
    seen = set()
    for record in entries:
        if not isinstance(record, dict):
            continue
        mid = record.get('id')
        if not isinstance(mid, str) or not mid or mid in seen or len(mid) > 300:
            continue
        seen.add(mid)
        evidence = free_classification(record)
        if not include_paid and evidence is None:
            continue
        arch = record.get('architecture') if isinstance(record.get('architecture'), dict) else {}
        output = arch.get('output_modalities')
        if isinstance(output, list) and 'text' not in output:
            continue
        params = record.get('supported_parameters')
        if not isinstance(params, list):
            params = []
        # OpenAI-compatible transport does NOT itself prove per-model tool support.
        params = [v for v in params if isinstance(v, str)]
        inputs = arch.get('input_modalities')
        if not isinstance(inputs, list):
            inputs = ['text']
        top = record.get('top_provider') if isinstance(record.get('top_provider'), dict) else {}
        maximum = record.get('max_completion_tokens') or top.get('max_completion_tokens')
        if not isinstance(maximum, int) or isinstance(maximum, bool) or maximum <= 0:
            maximum = None
        context = record.get('context_length')
        if not isinstance(context, int) or isinstance(context, bool) or context < 0:
            context = 0
        name = record.get('name') if isinstance(record.get('name'), str) else mid
        desc = record.get('description') if isinstance(record.get('description'), str) else ''
        result.append({
            'provider': 'kilo-gateway', 'id': mid, 'route_id': 'kilo/' + mid,
            'name': name, 'description': desc[:1600],
            'context_length': context, 'max_completion_tokens': maximum,
            'input_modalities': inputs, 'supported_parameters': params,
            'endpoint': CHAT_URL, 'pricing': record.get('pricing'),
            'evidence': evidence or 'not-free-or-unverified',
            'routable': bool(evidence),
            'risk_note': ('Auto Free may send prompts to providers that retain training data; do not use sensitive data.'
                          if mid == 'kilo-auto/free' else ''),
            'catalog_source': CATALOG_URL,
        })
    result.sort(key=lambda m: (not m['routable'], -m['context_length'], m['id']))
    return result


def fetch_catalog(timeout=18):
    req = urllib.request.Request(CATALOG_URL, headers={
        'Accept': 'application/json', 'User-Agent': 'ZFree-Kilo-Model-Explorer/1.0'})
    with urllib.request.urlopen(req, timeout=timeout) as response:
        if response.status != 200:
            raise ValueError('Kilo catalog HTTP status ' + str(response.status))
        # Bounded read prevents unexpected large responses from exhausting memory.
        raw = response.read(8 * 1024 * 1024 + 1)
        if len(raw) > 8 * 1024 * 1024:
            raise ValueError('Kilo catalog too large')
        return json.loads(raw)


def discover():
    return extract_models(fetch_catalog())


def query(query='', *, include_paid=False, limit=500):
    models = extract_models(fetch_catalog(), include_paid=include_paid)
    if query:
        needle = query.casefold()
        models = [m for m in models if needle in ' '.join((m['id'], m['name'], m['description'])).casefold()]
    return models[:max(0, limit)]


def main(argv=None):
    ap = argparse.ArgumentParser(description='Live Kilo Gateway free model list/query; never invokes inference')
    ap.add_argument('--query', default='', help='Case-insensitive search of ID, name, or description')
    ap.add_argument('--all', action='store_true', help='Include paid/unverified catalog entries for inspection only')
    ap.add_argument('--json', action='store_true', help='Emit machine-readable details')
    ap.add_argument('--limit', type=int, default=500)
    args = ap.parse_args(argv)
    try:
        results = query(args.query, include_paid=args.all, limit=args.limit)
    except (OSError, ValueError, json.JSONDecodeError) as exc:
        print(f'Kilo catalog unavailable: {type(exc).__name__}: {exc}', file=sys.stderr)
        return 2
    if args.json:
        print(json.dumps({'source': CATALOG_URL, 'free_only': not args.all,
                          'count': len(results), 'models': results}, indent=2))
    else:
        for m in results:
            state = 'FREE' if m['routable'] else 'NOT-VERIFIED-FREE'
            print(f"{state:18} {m['id']:62} {m['context_length']:>8}  {m['name']}")
        print(f'{len(results)} model(s); no inference performed')
    return 0


if __name__ == '__main__':
    raise SystemExit(main())