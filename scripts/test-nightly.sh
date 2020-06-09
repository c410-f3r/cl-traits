#!/usr/bin/env bash

. "$(dirname "$0")/test-common.sh" --source-only

test_package_with_feature "cl-traits" "const-generics"
#test_package_with_feature "cl-traits" "with-staticvec"

#test_package_generic "cl-traits"
