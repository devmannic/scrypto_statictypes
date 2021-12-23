#!/usr/bin/env sh
set -e

# test iwth compile time only features
cargo build --target wasm32-unknown-unknown --release --no-default-features
cargo test --release --no-default-features

# test with default (runtime typechecks) features
cargo build --target wasm32-unknown-unknown --release
cargo test --release
