// @ts-check
/**
 * @hq:context The reserved and forbidden path data for the Labs workspace: the eight TierFlow Wave 3 lane
 *   source/test pairs, the integrator-owned integration-2 trees, the repos_hq L0 shared-serial files and
 *   integration docs, and the Harness manager-merge-only paths. Labs refuses to write any of them.
 * @hq:owner labs
 * @hq:task LABS-W3-000
 * @hq:why this is DATA and not a hardcoded rule: the ownership record lives in the owning repositories and
 *   can change there. Labs reads the record, cites the document and line it came from, and never restates
 *   an ownership decision as its own authority.
 * @hq:invariant every entry names its owner and its source document line, so any refusal can be checked
 *   against the owning repository instead of being trusted.
 * @hq:invariant a path only means something inside a repository. `/tests/x` in vivim-labs is not TierFlow's
 *   `/tests/x`, so a forbidden entry is matched against the repository the work item is scoped to.
 * @hq:see vivim-ui VIVIM-VISUAL-RUNTIME/WAVE-2/WAVE-3-OWNERSHIP.md:9-18
 * @hq:see repos_hq docs/integration/archive/lanes.v1.json:23-34,52
 * @hq:see repos_hq docs/integration/archive/VIVIM-INTEGRATION-ROADMAP.md:358-359
 * @hq:see docs/WAVE-3-READINESS-HANDOFF.md:159-193
 */

/**
 * @typedef {Object} ForbiddenPath
 * @property {string} repo       Repository the path lives in. A path string is meaningless without it.
 * @property {string} path       Repository-anchored absolute path (`/a/b/c`) or a `/**` glob.
 * @property {string} owner      Who owns the path. The owner, not Labs, decides what happens there.
 * @property {string} kind       What kind of reservation this is.
 * @property {string} source     Document and line the reservation was read from.
 */

/** @type {readonly ForbiddenPath[]} */
export const FORBIDDEN_PATHS = Object.freeze([
  // ── TierFlow (vivim-ui): Wave 3 lane source/test pairs ──────────────────────
  // Source: VIVIM-VISUAL-RUNTIME/WAVE-2/WAVE-3-OWNERSHIP.md table, rows A..H.
  {
    repo: "vivim-ui",
    path: "/src/visual-runtime/document/wave3-action-semantics.ts",
    owner: "tierflow-wave3-lane-a",
    kind: "reserved-source",
    source: "vivim-ui VIVIM-VISUAL-RUNTIME/WAVE-2/WAVE-3-OWNERSHIP.md:9",
  },
  {
    repo: "vivim-ui",
    path: "/tests/visual-runtime/wave3-a-schema.test.ts",
    owner: "tierflow-wave3-lane-a",
    kind: "reserved-test",
    source: "vivim-ui VIVIM-VISUAL-RUNTIME/WAVE-2/WAVE-3-OWNERSHIP.md:9",
  },
  {
    repo: "vivim-ui",
    path: "/src/visual-runtime/components/wave3-canonical-map.ts",
    owner: "tierflow-wave3-lane-b",
    kind: "reserved-source",
    source: "vivim-ui VIVIM-VISUAL-RUNTIME/WAVE-2/WAVE-3-OWNERSHIP.md:10",
  },
  {
    repo: "vivim-ui",
    path: "/tests/visual-runtime/wave3-b-components.test.ts",
    owner: "tierflow-wave3-lane-b",
    kind: "reserved-test",
    source: "vivim-ui VIVIM-VISUAL-RUNTIME/WAVE-2/WAVE-3-OWNERSHIP.md:10",
  },
  {
    repo: "vivim-ui",
    path: "/src/visual-runtime/revisions/wave3-atomic-replay.ts",
    owner: "tierflow-wave3-lane-c",
    kind: "reserved-source",
    source: "vivim-ui VIVIM-VISUAL-RUNTIME/WAVE-2/WAVE-3-OWNERSHIP.md:11",
  },
  {
    repo: "vivim-ui",
    path: "/tests/visual-runtime/wave3-c-replay.test.ts",
    owner: "tierflow-wave3-lane-c",
    kind: "reserved-test",
    source: "vivim-ui VIVIM-VISUAL-RUNTIME/WAVE-2/WAVE-3-OWNERSHIP.md:11",
  },
  {
    repo: "vivim-ui",
    path: "/src/visual-runtime/render/wave3-semantics.ts",
    owner: "tierflow-wave3-lane-d",
    kind: "reserved-source",
    source: "vivim-ui VIVIM-VISUAL-RUNTIME/WAVE-2/WAVE-3-OWNERSHIP.md:12",
  },
  {
    repo: "vivim-ui",
    path: "/tests/visual-runtime/wave3-d-render.test.ts",
    owner: "tierflow-wave3-lane-d",
    kind: "reserved-test",
    source: "vivim-ui VIVIM-VISUAL-RUNTIME/WAVE-2/WAVE-3-OWNERSHIP.md:12",
  },
  {
    repo: "vivim-ui",
    path: "/src/visual-runtime/experience/wave3-journey-fixture.ts",
    owner: "tierflow-wave3-lane-e",
    kind: "reserved-source",
    source: "vivim-ui VIVIM-VISUAL-RUNTIME/WAVE-2/WAVE-3-OWNERSHIP.md:13",
  },
  {
    repo: "vivim-ui",
    path: "/tests/visual-runtime/wave3-e-experience.test.ts",
    owner: "tierflow-wave3-lane-e",
    kind: "reserved-test",
    source: "vivim-ui VIVIM-VISUAL-RUNTIME/WAVE-2/WAVE-3-OWNERSHIP.md:13",
  },
  {
    repo: "vivim-ui",
    path: "/src/visual-runtime/knowledge/wave3-feed-projection.ts",
    owner: "tierflow-wave3-lane-f",
    kind: "reserved-source",
    source: "vivim-ui VIVIM-VISUAL-RUNTIME/WAVE-2/WAVE-3-OWNERSHIP.md:14",
  },
  {
    repo: "vivim-ui",
    path: "/tests/visual-runtime/wave3-f-knowledge.test.ts",
    owner: "tierflow-wave3-lane-f",
    kind: "reserved-test",
    source: "vivim-ui VIVIM-VISUAL-RUNTIME/WAVE-2/WAVE-3-OWNERSHIP.md:14",
  },
  {
    repo: "vivim-ui",
    path: "/src/visual-runtime/command/wave3-descriptor-proof.ts",
    owner: "tierflow-wave3-lane-g",
    kind: "reserved-source",
    source: "vivim-ui VIVIM-VISUAL-RUNTIME/WAVE-2/WAVE-3-OWNERSHIP.md:15",
  },
  {
    repo: "vivim-ui",
    path: "/tests/visual-runtime/wave3-g-command.test.ts",
    owner: "tierflow-wave3-lane-g",
    kind: "reserved-test",
    source: "vivim-ui VIVIM-VISUAL-RUNTIME/WAVE-2/WAVE-3-OWNERSHIP.md:15",
  },
  {
    repo: "vivim-ui",
    path: "/src/visual-runtime/host/wave3-read-gateway-fixture.ts",
    owner: "tierflow-wave3-lane-h",
    kind: "reserved-source",
    source: "vivim-ui VIVIM-VISUAL-RUNTIME/WAVE-2/WAVE-3-OWNERSHIP.md:16",
  },
  {
    repo: "vivim-ui",
    path: "/tests/visual-runtime/wave3-h-host.test.ts",
    owner: "tierflow-wave3-lane-h",
    kind: "reserved-test",
    source: "vivim-ui VIVIM-VISUAL-RUNTIME/WAVE-2/WAVE-3-OWNERSHIP.md:16",
  },

  // ── TierFlow (vivim-ui): the integrator alone owns the integration-2 trees ──
  // Source: WAVE-3-OWNERSHIP.md:18 — "Integrator alone: src/visual-runtime/integration-2/**,
  // tests/visual-runtime/integration-2/**, and later new explicit Wave-3 integration-owned paths".
  {
    repo: "vivim-ui",
    path: "/src/visual-runtime/integration-2/**",
    owner: "tierflow-integrator",
    kind: "reserved-source",
    source: "vivim-ui VIVIM-VISUAL-RUNTIME/WAVE-2/WAVE-3-OWNERSHIP.md:18",
  },
  {
    repo: "vivim-ui",
    path: "/tests/visual-runtime/integration-2/**",
    owner: "tierflow-integrator",
    kind: "reserved-test",
    source: "vivim-ui VIVIM-VISUAL-RUNTIME/WAVE-2/WAVE-3-OWNERSHIP.md:18",
  },

  // ── TierFlow (vivim-ui): each lane's own handoff and evidence paths ─────────
  // Source: WAVE-3-OWNERSHIP.md:18 — "Each Wave 3 lane's handoff/evidence goes only to its own newly
  // named WAVE-3/LANE-X-HANDOFF.md and WAVE-3/evidence/X-* paths." The Wave 3 directory in the repository
  // is VIVIM-VISUAL-RUNTIME/WAVE-3, so the repository-anchored paths are written that way.
  ...[
    ["a", "A"],
    ["b", "B"],
    ["c", "C"],
    ["d", "D"],
    ["e", "E"],
    ["f", "F"],
    ["g", "G"],
    ["h", "H"],
  ].map(([slug, lane]) => ({
    repo: "vivim-ui",
    path: `/VIVIM-VISUAL-RUNTIME/WAVE-3/LANE-${lane}-HANDOFF.md`,
    owner: `tierflow-wave3-lane-${slug}`,
    kind: "reserved-handoff",
    source: "vivim-ui VIVIM-VISUAL-RUNTIME/WAVE-2/WAVE-3-OWNERSHIP.md:18",
  })),
  ...[
    ["a", "A"],
    ["b", "B"],
    ["c", "C"],
    ["d", "D"],
    ["e", "E"],
    ["f", "F"],
    ["g", "G"],
    ["h", "H"],
  ].map(([slug, lane]) => ({
    repo: "vivim-ui",
    path: `/VIVIM-VISUAL-RUNTIME/WAVE-3/evidence/${lane}-*`,
    owner: `tierflow-wave3-lane-${slug}`,
    kind: "reserved-evidence",
    source: "vivim-ui VIVIM-VISUAL-RUNTIME/WAVE-2/WAVE-3-OWNERSHIP.md:18",
  })),

  // ── repos_hq: the six shared-serial files only the integrator (L0) may edit ──
  // Source: docs/integration/archive/lanes.v1.json sharedSerial.paths:26-32 and
  // docs/integration/archive/VIVIM-INTEGRATION-ROADMAP.md:358-359.
  {
    repo: "repos_hq",
    path: "/hq/bin/hq.mjs",
    owner: "repos_hq L0 (integrator)",
    kind: "shared-serial",
    source: "repos_hq docs/integration/archive/lanes.v1.json:26",
  },
  {
    repo: "repos_hq",
    path: "/hq/lib/schema.mjs",
    owner: "repos_hq L0 (integrator)",
    kind: "shared-serial",
    source: "repos_hq docs/integration/archive/lanes.v1.json:27",
  },
  {
    repo: "repos_hq",
    path: "/package.json",
    owner: "repos_hq L0 (integrator)",
    kind: "shared-serial",
    source: "repos_hq docs/integration/archive/lanes.v1.json:28",
  },
  {
    repo: "repos_hq",
    path: "/docs/VIVIM-INTEGRATION-ROADMAP.md",
    owner: "repos_hq L0 (integrator)",
    kind: "shared-serial",
    source: "repos_hq docs/integration/archive/lanes.v1.json:29",
  },
  {
    repo: "repos_hq",
    path: "/docs/integration/gap-register.v1.json",
    owner: "repos_hq L0 (integrator)",
    kind: "shared-serial-ledger",
    source: "repos_hq docs/integration/archive/lanes.v1.json:30",
  },
  {
    repo: "repos_hq",
    path: "/docs/integration/lanes.v1.json",
    owner: "repos_hq L0 (integrator)",
    kind: "shared-serial-ledger",
    source: "repos_hq docs/integration/archive/lanes.v1.json:31",
  },
  {
    repo: "repos_hq",
    path: "/docs/integration/TEAM.v1.md",
    owner: "repos_hq L0 (integrator)",
    kind: "shared-serial-ledger",
    source: "repos_hq docs/integration/archive/lanes.v1.json:32",
  },
  {
    repo: "repos_hq",
    path: "/docs/integration/**",
    owner: "repos_hq L0 (integrator)",
    kind: "reserved-docs",
    source: "repos_hq docs/integration/archive/lanes.v1.json:52",
  },

  // ── vivim-web-harness: manager-merge-only paths ─────────────────────────────
  // Source: docs/WAVE-3-READINESS-HANDOFF.md:190-193 — "the manager-merge-only paths
  // (studio/src/studio/chat-channel.mjs, studio/bin/commands/chat.mjs, schemas/chat-*.v1.json,
  // docs/integration/STREAM-BOUNDARIES.md) are untouched."
  {
    repo: "vivim-web-harness",
    path: "/studio/src/studio/chat-channel.mjs",
    owner: "vivim-web-harness channel-manager",
    kind: "manager-merge-only",
    source: "vivim-labs docs/WAVE-3-READINESS-HANDOFF.md:190-193",
  },
  {
    repo: "vivim-web-harness",
    path: "/studio/bin/commands/chat.mjs",
    owner: "vivim-web-harness channel-manager",
    kind: "manager-merge-only",
    source: "vivim-labs docs/WAVE-3-READINESS-HANDOFF.md:190-193",
  },
  {
    repo: "vivim-web-harness",
    path: "/schemas/chat-*.v1.json",
    owner: "vivim-web-harness channel-manager",
    kind: "manager-merge-only",
    source: "vivim-labs docs/WAVE-3-READINESS-HANDOFF.md:190-193",
  },
  {
    repo: "vivim-web-harness",
    path: "/docs/integration/STREAM-BOUNDARIES.md",
    owner: "vivim-web-harness channel-manager",
    kind: "manager-merge-only",
    source: "vivim-labs docs/WAVE-3-READINESS-HANDOFF.md:190-193",
  },
]);

/**
 * The paths a Labs work item may write. Labs owns experiment and evidence work inside its own
 * repository; it never writes another repository's source, because it cannot grant itself source
 * authority (docs/ENGINEERING-EVIDENCE-FACTORY.md, "Canonical integration authority").
 * @type {readonly string[]}
 */
export const LABS_WRITE_ROOTS = Object.freeze(["/labs", "/work", "/tests"]);

/**
 * Root files a Labs work item may touch inside vivim-labs. They are listed because a bare `/package.json`
 * is also a reserved path in another repository, and a path only means something inside its repository.
 * @type {readonly string[]}
 */
export const LABS_ROOT_FILE_EXCEPTIONS = Object.freeze(["/package.json", "/tsconfig.json"]);

/** Repositories a Labs work item may name. @type {readonly string[]} */
export const KNOWN_REPOS = Object.freeze([
  "vivim-labs",
  "vivim-ui",
  "repos_hq",
  "vivim-web-harness",
]);

/**
 * Is this a repository-anchored absolute path? The rule is POSIX and literal: it starts with `/`, it has
 * no `..` segment, no backslash and no drive letter. `C:/x` and `labs/x` are both refused.
 * @param {unknown} value
 * @returns {boolean}
 */
export function isRepoAbsolutePath(value) {
  if (typeof value !== "string" || value.length === 0) return false;
  if (!value.startsWith("/")) return false;
  if (value.includes("\\")) return false;
  if (/^[A-Za-z]:/.test(value)) return false;
  const segments = value.split("/");
  if (segments.includes("..")) return false;
  return true;
}

/**
 * Split a repository path into segments, dropping the leading slash and any trailing slash.
 * @param {string} value
 * @returns {string[]}
 */
export function pathSegments(value) {
  return value.split("/").filter((segment) => segment.length > 0);
}

/**
 * The segment prefix a path claims, with any glob tail removed. `/a/b/**` claims `/a/b`.
 * @param {string} value
 * @returns {string[]}
 */
export function claimPrefix(value) {
  const segments = pathSegments(value);
  const prefix = [];
  for (const segment of segments) {
    if (segment.includes("*")) break;
    prefix.push(segment);
  }
  return prefix;
}

/**
 * Does `path` fall inside `owner`? Both are repository paths; `owner` may end in `/**` or contain a
 * trailing `*` wildcard. The comparison is segment-wise, so `/a/bc` is not inside `/a/b`.
 * @param {string} path
 * @param {string} owner
 * @returns {boolean}
 */
export function pathWithin(path, owner) {
  const ownerRaw = owner.endsWith("/**") ? owner.slice(0, -3) : owner.replace(/\*+$/, "");
  const ownerSegments = pathSegments(ownerRaw);
  const pathParts = pathSegments(path);
  if (ownerSegments.length === 0) return true;
  if (pathParts.length < ownerSegments.length) return false;
  for (let index = 0; index < ownerSegments.length; index += 1) {
    const ownerSegment = ownerSegments[index];
    const pathSegment = pathParts[index];
    if (ownerSegment.includes("*")) {
      const pattern = new RegExp(
        `^${ownerSegment.split("*").map((part) => part.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")).join(".*")}$`,
      );
      if (!pattern.test(pathSegment)) return false;
      continue;
    }
    if (ownerSegment !== pathSegment) return false;
  }
  return true;
}

/**
 * The forbidden entry a path falls inside, for the given repository, or null.
 * @param {string} repo
 * @param {string} path
 * @returns {ForbiddenPath | null}
 */
export function findForbiddenPath(repo, path) {
  for (const entry of FORBIDDEN_PATHS) {
    if (entry.repo !== repo) continue;
    if (pathWithin(path, entry.path)) return entry;
    // The reverse direction: the item claims a parent directory that contains the reserved path.
    if (pathWithin(entry.path, path)) return entry;
  }
  return null;
}

/**
 * Is the path inside one of the Labs write roots, or one of the Labs root file exceptions?
 * @param {string} path
 * @returns {boolean}
 */
export function isLabsWritablePath(path) {
  for (const root of LABS_WRITE_ROOTS) {
    if (pathWithin(path, root)) return true;
  }
  return LABS_ROOT_FILE_EXCEPTIONS.includes(path);
}
