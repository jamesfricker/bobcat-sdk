#!/bin/sh -eu

cargo stylus deploy \
	--wasm-file bozo.wasm \
	--private-key "$BOZO_PRIVATE_KEY" \
	--endpoint "$BOZO_ENDPOINT" \
	--no-verify \
	        | sed -nr 's/.*deployed code at address: +.*(0x.{40}).*$/\1/p'
