#!/bin/sh -u

wasm-opt \
	--dce \
	--rse \
	--signature-pruning \
	--strip-debug \
	--enable-bulk-memory \
	--strip-producers \
	--strip \
	-Oz \
	-o $2 \
	$1
