#!/bin/sh -e

# cargo nextest run --features std

make

arbos-forge test --stylus-debug -vvv
