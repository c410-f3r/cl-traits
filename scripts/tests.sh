#!/usr/bin/env bash

set -euxo pipefail

cargo install --git https://github.com/c410-f3r/rust-tools --force

rt='rust-tools --template you-rust'

export CARGO_TARGET_DIR="$($rt target-dir)"
export RUST_BACKTRACE=1
export RUSTFLAGS="$($rt rust-flags)"

$rt clippy -Aclippy::default_numeric_fallback
$rt rustfmt

$rt test-generic cl-traits
$rt test-with-features cl-traits alloc
$rt test-with-features cl-traits std
$rt test-with-features cl-traits with-arrayvec
$rt test-with-features cl-traits with-smallvec
$rt test-with-features cl-traits with-staticvec
$rt test-with-features cl-traits with-tinyvec
