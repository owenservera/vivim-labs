#!/data/data/com.termux/files/usr/bin/bash
# Scoped VIVIM Android development defaults. No global shell or repo mutations.
set -e
export CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-1}"
export CARGO_PROFILE_DEV_DEBUG="${CARGO_PROFILE_DEV_DEBUG:-0}"
export CARGO_PROFILE_TEST_DEBUG="${CARGO_PROFILE_TEST_DEBUG:-0}"
export CARGO_INCREMENTAL="${CARGO_INCREMENTAL:-0}"
export SCCACHE_CACHE_SIZE="${SCCACHE_CACHE_SIZE:-512M}"
export RUST_TEST_THREADS="${RUST_TEST_THREADS:-1}"
# Opt in after cache server verification; native rustc is the reliable default.
if [ "${VIVIM_USE_SCCACHE:-0}" = 1 ]; then
  export RUSTC_WRAPPER="$(command -v sccache)"
fi
cd "$(dirname "$0")/vivim-product"
exec "$@"
