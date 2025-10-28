#!/bin/sh

features=std,proptest-enabled,alloy-enabled,alloc

export PROPTEST_CASES=10

cargo mutants --features $features
