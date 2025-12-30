#!/bin/sh -eu

tmp=$(mktemp "${TMPDIR:-/tmp}/wasm-post.XXXXXX")
f="${tmp}.wasm1"

cleanup() {
	rm -f "$f" "$f.wat"
}
trap cleanup EXIT

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
