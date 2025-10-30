#!/bin/sh -e

url=${ENDPOINT:-https://testnet-rpc.superposition.so}

if [ -z "$ADMIN_ADDR" ]; then
	>&2 echo "ADMIN_ADDR unset"
	exit 2
fi

if [ -z "$PRIVATE_KEY" ]; then
	>&2 echo "PRIVATE_KEY unset"
	exit 2
fi

if [ -z "$BOZO_IMPL" ]; then
	BOZO_IMPL="$(./deploy.sh)"
	>&2 echo "BOZO_IMPL=$BOZO_IMPL"
fi

BOZO_IMPL="$(echo $BOZO_IMPL | sed -s 's/0x//g')"

proxy_bytecode="$(\
	huffc -b src/eip1967-constructor-args.huff \
		| sed "s/1000000000000000000000000000000000000001/$BOZO_IMPL/g")"

create_code="$(echo $proxy_bytecode$(cast calldata 'initialise(address)' "$ADMIN_ADDR" | sed -s 's/^0x//g'))"

cast send \
	--rpc-url "$url" \
	--private-key "$PRIVATE_KEY" \
	--create "$create_code" \
	--json \
		| jq -r .contractAddress
