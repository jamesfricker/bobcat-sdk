#!/bin/sh -e

features=std,proptest,alloy-enabled,alloc

cargo test --features $features -- --nocapture $@

cd examples

make

cd ../e2e-test

./tests.sh
