#!/usr/bin/env bash
# HQKGX1 record-order regression gate runner (keeper evidence helper).
# Each gate runs as its own command; each exit code is captured separately.
# No piping masks an exit code: output goes to a per-gate log file.
set -u
CRATE="/c/0-BlackBoxProject-0/repos_hq/research/rust-knowledge-graph"
EV="/c/0-BlackBoxProject-0/vivim-labs/agents/workspaces/rust-core-keeper/evidence"
STAMP="${1:-20261111-hqkgx1-green}"

# Environment-only repair: rustc's MSVC linker discovery stopped resolving
# after the VS18 tree was rewritten on 2026-11-11. Point cargo at the MSVC
# link.exe that is still installed and provide the CRT/SDK library paths.
# No repo file is modified by this.
export LIB="C:\\Program Files\\Microsoft Visual Studio\\18\\Community\\VC\\Tools\\MSVC\\14.51.36231\\lib\\x64;C:\\Program Files (x86)\\Windows Kits\\10\\Lib\\10.0.26100.0\\um\\x64;C:\\Program Files (x86)\\Windows Kits\\10\\Lib\\10.0.26100.0\\ucrt\\x64"
export CARGO_TARGET_X86_64_PC_WINDOWS_MSVC_LINKER="C:\\Program Files\\Microsoft Visual Studio\\18\\Community\\VC\\Tools\\MSVC\\14.51.36231\\bin\\Hostx64\\x64\\link.exe"

cd "$CRATE" || exit 9

run_gate() {
  local name="$1"; shift
  local out="$EV/${STAMP}-${name}.log"
  echo "=== GATE START ${name}: $* ==="
  "$@" > "$out" 2>&1
  local rc=$?
  echo "GATE ${name} EXIT=${rc}"
  grep -E "^test result:|^\s+Running |^error(\[|:)" "$out" | tail -40
  echo "GATE ${name} LOG=${out}"
  return 0
}

run_gate fmt cargo fmt --check
run_gate check cargo check
run_gate test-import cargo test --features importer --test import
run_gate test-importer cargo test --features importer
run_gate build-importer cargo build --features importer
echo "=== GATES COMPLETE ==="
