#!/usr/bin/env bash
set -e

test_package_generic() {
    local package=$1

    /bin/echo -e "\e[0;33m***** Testing ${package} without features *****\e[0m\n"
    cargo test --lib --manifest-path "${package}"/Cargo.toml --no-default-features --tests 

    /bin/echo -e "\e[0;33m***** Testing ${package} with all features *****\e[0m\n"
    cargo test --all-features --lib --manifest-path "${package}"/Cargo.toml --no-default-features --tests
}

test_package_with_examples() {
    local package=$1

    /bin/echo -e "\e[0;33m***** Testing ${package} examples *****\e[0m\n"
    cargo test --all-features --manifest-path "${package}"/Cargo.toml --no-default-features
}

test_package_generic "cl-traits"
test_package_generic "cl-traits-derive"
test_package_with_examples "cl-traits"
