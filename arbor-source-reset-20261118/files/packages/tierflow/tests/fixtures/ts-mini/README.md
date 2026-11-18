# ts-mini

A minimal TypeScript/Bun project that has nothing to do with TierFlow.

- no git repository
- no HQ tooling, no `.hq`, no index
- no dependencies, no `node_modules`, no install step
- imports only `bun:test`

Scripts:

```sh
bun run test        # green suite  -> exit 0
bun run test:fail   # red suite    -> exit 1 (failure propagation proof)
```

The test files use a `.check.ts` suffix on purpose. Bun's test discovery only
picks up names containing `.test`, `.spec`, `_test_` or `_spec_`, so the parent
repository's `bun test tests/` never picks these fixtures up; they are invoked
by explicit path instead.
