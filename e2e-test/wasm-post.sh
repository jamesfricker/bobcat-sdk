#!/bin/sh -u

tmpdir="${TMPDIR:-/tmp}"
f=$(mktemp "${tmpdir%/}/bobcat-wasm.XXXXXX")

wasm-opt \
	--dce \
	--rse \
	--signature-pruning \
	--strip-debug \
	--enable-bulk-memory \
	--strip-producers \
	--strip \
	-Oz \
	-o "$f" \
	"$1"

wasm2wat "$f" > "$f.wat"

wat2wasm "$f.wat" -o "$2"
