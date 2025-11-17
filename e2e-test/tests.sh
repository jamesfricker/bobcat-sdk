#!/bin/sh -e

make

arbos-forge test --gas-snapshot-check true $@
