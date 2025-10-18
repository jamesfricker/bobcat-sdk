#!/bin/sh -e

features=std,proptest-enabled,alloy-enabled,alloc

cargo test --features $features -- --nocapture $@

cargo mutants --features $features

cd examples

cargo build --target wasm32-unknown-unknown --release

cd ../e2e-test

./tests.sh

cd ..

./check-codesize.sh
