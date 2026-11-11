#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
SLOT="${CARGO_SLOT:-$(command -v cargo-slot || true)}"   # compile slots where available; plain cargo elsewhere
$SLOT cargo fmt --all -- --check
$SLOT cargo clippy --workspace --all-targets --all-features -- -D warnings
$SLOT cargo test --workspace --all-features
