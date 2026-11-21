/**
 * @hq:action vivim.status.show@1
 * @hq:capability vivim.status.read@1
 * @hq:contract vivim.status.request@1
 * @hq:effect host:read-only
 * @hq:evidence test:tests/status.test.mjs
 */
export function showStatus() {}
