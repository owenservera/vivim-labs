# py-mini

A minimal Python project that has nothing to do with TierFlow.

- no git repository
- no HQ tooling, no `.hq`, no index
- standard library only, no virtualenv, no install step

Suites:

```sh
python -m unittest -v test_calc   # green suite -> exit 0
python -m unittest -v test_fail   # red suite   -> exit 1
```

The suites are selected by module name on purpose: `unittest discover` would
sweep in `test_fail.py` and turn the green run red.
