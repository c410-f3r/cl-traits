#!/usr/bin/env bash

set -eux

export RUST_BACKTRACE=full
export RUSTFLAGS='
    -D bad_style
    -D future_incompatible
    -D missing_debug_implementations
    -D missing_docs
    -D nonstandard_style
    -D rust_2018_compatibility
    -D rust_2018_idioms
    -D trivial_casts
    -D unused_lifetimes
    -D unused_qualifications
    -D warnings
'

clippy() {
    local package=$1
    local features=$2

    /bin/echo -e "\e[0;33m***** Running clippy on ${package} | ${features} *****\e[0m\n"
    cargo clippy $features --manifest-path "${package}"/Cargo.toml -- \
        -D clippy::restriction \
        -D warnings \
        -A clippy::implicit_return \
        -A clippy::missing_docs_in_private_items \
        -A clippy::missing_inline_in_public_items
}

run_package_example() {
    local package=$1
    local example=$2

    /bin/echo -e "\e[0;33m***** Running example ${example} of ${package} *****\e[0m\n"
    cargo run --all-features --example $example --manifest-path "${package}"/Cargo.toml
}

test_package_generic() {
    local package=$1

    /bin/echo -e "\e[0;33m***** Testing ${package} | --no-default-features *****\e[0m\n"
    cargo test --manifest-path "${package}"/Cargo.toml --no-default-features

    clippy $package "--no-default-features"

    /bin/echo -e "\e[0;33m***** Testing ${package} | --all-features *****\e[0m\n"
    cargo test --all-features --manifest-path "${package}"/Cargo.toml

    clippy $package "--all-features"
}

test_package_with_feature() {
    local package=$1
    local features=$2

    /bin/echo -e "\e[0;33m***** Testing ${package} | ${features} *****\e[0m\n"
    cargo test --manifest-path "${package}"/Cargo.toml --features "${features}" --no-default-features

    clippy $package "--features ${features}"
}

cargo fmt --all -- --check

test_package_generic "cl-traits"

test_package_with_feature "cl-traits" "alloc"
test_package_with_feature "cl-traits" "std"
test_package_with_feature "cl-traits" "with-arrayvec"
test_package_with_feature "cl-traits" "with-smallvec"
test_package_with_feature "cl-traits" "with-staticvec"
test_package_with_feature "cl-traits" "with-tinyvec"

run_package_example "cl-traits-examples" "manual"
