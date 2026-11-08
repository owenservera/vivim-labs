/**
 * @hq:context Ambient declarations for the runtime builtins this Labs scaffold uses. There is no
 *   `@types/node` dependency on purpose: the Labs contract is "TypeScript devDep only", so the handful of
 *   Node builtins the scaffold touches are declared here instead of installed.
 * @hq:owner labs
 * @hq:task LABS-W3-000
 * @hq:why a hand-written, honest surface beats a silently missing type package. If a module here drifts
 *   from the real Node API the typecheck fails loudly, which is the failure mode we want.
 * @hq:see tsconfig.json
 * @hq:see package.json
 */

declare module "node:fs" {
  export interface Stats {
    size: number;
    mtimeMs: number;
    isFile(): boolean;
    isDirectory(): boolean;
  }
  export function readFileSync(path: string, encoding: "utf8"): string;
  export function writeFileSync(path: string, data: string, encoding?: string): void;
  export function existsSync(path: string): boolean;
  export function mkdirSync(path: string, options?: { recursive?: boolean }): string | undefined;
  export function renameSync(from: string, to: string): void;
  export function unlinkSync(path: string): void;
  export function statSync(path: string): Stats;
  export function readdirSync(path: string): string[];
  export function openSync(path: string, flags: string): number;
  export function closeSync(fd: number): void;
  export function writeSync(fd: number, data: string): number;
  export function statfsSync(path: string): {
    bsize: number;
    blocks: number;
    bfree: number;
    bavail: number;
  };
  export function mkdtempSync(prefix: string): string;
}

declare module "node:path" {
  export function join(...parts: string[]): string;
  export function resolve(...parts: string[]): string;
  export function dirname(target: string): string;
  export function basename(target: string, ext?: string): string;
  export function relative(from: string, to: string): string;
  export function isAbsolute(target: string): boolean;
  export function normalize(target: string): string;
  export const sep: string;
}

declare module "node:os" {
  export interface CpuInfo {
    model: string;
    speed: number;
  }
  export function cpus(): CpuInfo[];
  export function totalmem(): number;
  export function freemem(): number;
  export function platform(): string;
  export function release(): string;
  export function tmpdir(): string;
  export function homedir(): string;
}

declare module "node:child_process" {
  export interface SpawnSyncOptions {
    cwd?: string;
    encoding?: string;
    timeout?: number;
    windowsHide?: boolean;
    shell?: boolean;
    maxBuffer?: number;
    env?: Record<string, string | undefined>;
  }
  export interface SpawnSyncResult {
    status: number | null;
    signal: string | null;
    stdout: string;
    stderr: string;
    pid: number;
    error?: Error;
  }
  export function spawnSync(command: string, options?: SpawnSyncOptions): SpawnSyncResult;
  export function spawnSync(
    command: string,
    args?: readonly string[],
    options?: SpawnSyncOptions,
  ): SpawnSyncResult;
}

declare module "node:crypto" {
  export interface Hash {
    update(data: string): Hash;
    digest(encoding: "hex"): string;
  }
  export function createHash(algorithm: string): Hash;
}

declare module "node:url" {
  export function fileURLToPath(url: string): string;
}

declare module "bun:test" {
  export interface Matchers {
    toBe(expected: unknown): void;
    toEqual(expected: unknown): void;
    toContain(expected: unknown): void;
    toMatch(expected: RegExp | string): void;
    toHaveLength(expected: number): void;
    toBeTruthy(): void;
    toBeFalsy(): void;
    toBeUndefined(): void;
    toBeDefined(): void;
    toBeNull(): void;
    toBeGreaterThan(expected: number): void;
    toBeGreaterThanOrEqual(expected: number): void;
    toBeLessThan(expected: number): void;
    toBeLessThanOrEqual(expected: number): void;
    toBeInstanceOf(expected: unknown): void;
    toThrow(expected?: unknown): void;
    not: Matchers;
  }
  export function expect(value: unknown): Matchers;
  export function test(name: string, fn: () => void | Promise<void>): void;
  export function describe(name: string, fn: () => void): void;
  export function beforeEach(fn: () => void): void;
  export function afterEach(fn: () => void): void;
}

declare const console: {
  log(...args: unknown[]): void;
  error(...args: unknown[]): void;
  warn(...args: unknown[]): void;
};

interface ImportMeta {
  url: string;
}

declare const process: {
  argv: string[];
  execPath: string;
  pid: number;
  platform: string;
  env: Record<string, string | undefined>;
  cwd(): string;
  exit(code?: number): never;
  exitCode: number | undefined;
  stdout: { write(chunk: string): boolean };
  stderr: { write(chunk: string): boolean };
};

declare function setTimeout(handler: () => void, timeout?: number): { unref(): void };
declare function clearTimeout(handle: unknown): void;
