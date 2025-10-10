#!/bin/sh

features=std,proptest-enabled,alloy-enabled

#cargo mutants --features $features

cargo test --features $features
