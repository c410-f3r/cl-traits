#!/usr/bin/env bash

. ./scripts/tests-common.sh --source-only

test_package_generic "cl-traits"

test_package_with_feature "cl-traits" "const_generics"
test_package_with_feature "cl-traits" "with-staticvec"

