#!/usr/bin/env bash
set -e

PACKAGES=(
    cl-traits-derive
    cl-traits
)

for package in "${PACKAGES[@]}"; do
    pushd "${package}"
    /bin/echo -e "\e[0;33m***** Publishing ${package} *****\e[0m\n"
    cargo publish
    popd
done
