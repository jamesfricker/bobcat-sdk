#!/bin/sh

features=std,proptest-enabled,alloy-enabled,alloc

#cargo mutants --features $features

cargo test --features $features -- --nocapture $@
