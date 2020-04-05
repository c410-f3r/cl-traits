#!/usr/bin/env bash

. ./scripts/tests-common.sh --source-only

test_package_generic "cl-traits-derive"

test_package_with_feature "cl-traits" "alloc"
test_package_with_feature "cl-traits" "std"
test_package_with_feature "cl-traits" "with-arrayvec"
test_package_with_feature "cl-traits" "with-derive"
test_package_with_feature "cl-traits" "with-serde"
test_package_with_feature "cl-traits" "with-smallvec"
test_package_with_feature "cl-traits" "with-tinyvec"

run_package_example "examples" "macros"
run_package_example "examples" "manual"

