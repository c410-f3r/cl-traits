#!/usr/bin/env bash

. "$(dirname "$0")/commons.sh" --source-only

run_package_example() {
    local package=$1
    local example=$2

    /bin/echo -e "\e[0;33m***** Running example ${example} of ${package}  *****\e[0m\n"
    cargo run --all-features --example $example --manifest-path "${package}"/Cargo.toml
}

test_package_generic() {
    local package=$1

    /bin/echo -e "\e[0;33m***** Testing ${package} without features *****\e[0m\n"
    cargo test --manifest-path "${package}"/Cargo.toml --no-default-features

    /bin/echo -e "\e[0;33m***** Testing ${package} with all features *****\e[0m\n"
    cargo test --all-features --manifest-path "${package}"/Cargo.toml
}

test_package_with_feature() {
    local package=$1
    local feature=$2

    /bin/echo -e "\e[0;33m***** Testing ${package} with feature '${feature}' *****\e[0m\n"
    cargo test --manifest-path "${package}"/Cargo.toml --features "${feature}" --no-default-features
}

test_package_generic "cl-traits"

test_package_with_feature "cl-traits" "alloc"
test_package_with_feature "cl-traits" "std"
test_package_with_feature "cl-traits" "with-arrayvec"
test_package_with_feature "cl-traits" "with-smallvec"
test_package_with_feature "cl-traits" "with-staticvec"
test_package_with_feature "cl-traits" "with-tinyvec"

run_package_example "cl-traits-examples" "manual"

