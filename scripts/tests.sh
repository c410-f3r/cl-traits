#!/usr/bin/env bash

. ./scripts/tests-common.sh --source-only

test_package_generic "cl-traits-derive"

test_package_with_feature "cl-traits" "alloc"
test_package_with_feature "cl-traits" "std"
test_package_with_feature "cl-traits" "with_derive"
test_package_with_feature "cl-traits" "with_serde"
test_package_with_feature "cl-traits" "with_arrayvec"
test_package_with_feature "cl-traits" "with_smallvec"

run_package_example "examples" "macros"
run_package_example "examples" "manual"

