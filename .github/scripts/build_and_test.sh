#!/usr/bin/env bash
set -x
cargo test --all-features
cargo fmt -- --check
cargo clippy --all-targets --all-features -- --deny warnings
cargo doc --no-deps
