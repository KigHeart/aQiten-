#!/usr/bin/env bash
# Local validation gate for the aQiten workspace. Run before every push.
# Appends a one-line result to docs/VALIDATION_LOG.md.
set -euo pipefail
cd "$(dirname "$0")/.."
export CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-2}"
step() { printf '\n== %s ==\n' "$1"; }
step "fmt";       cargo fmt --all -- --check
step "clippy";    cargo clippy --workspace --all-targets --locked -- -D warnings
step "test (debug, overflow checks on)";  cargo test --workspace --locked
step "test (release)";                    cargo test --workspace --release --locked
step "doc";       RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --locked
step "build core crates (declared no_std; bare-metal target check not run, target unavailable)"
cargo build --locked -p aqiten-error -p aqiten-codec -p aqiten-field
rev="$(git rev-parse --short HEAD 2>/dev/null || echo none)"
printf -- '- %s  rev %s  rustc %s  ci-local OK\n' "$(date -u +%FT%TZ)" "$rev" "$(rustc --version | cut -d' ' -f2)" >> docs/VALIDATION_LOG.md
echo "ALL GATES PASSED"
