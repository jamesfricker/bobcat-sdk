#!/bin/sh -e

if [ -z "$BOZO_ENDPOINT" ]; then
	>&2 echo "BOZO_ENDPOINT unset"
	exit 2
fi

if [ -z "$BOZO_ADMIN_ADDR" ]; then
	>&2 echo "BOZO_ADMIN_ADDR unset"
	exit 2
fi

if [ -z "$BOZO_PRIVATE_KEY" ]; then
	>&2 echo "BOZO_PRIVATE_KEY unset"
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

create_code="$(echo $proxy_bytecode$(cast calldata 'initialise(address)' "$BOZO_ADMIN_ADDR" | sed -s 's/^0x//g'))"

cast send \
	--rpc-url "$BOZO_ENDPOINT" \
	--private-key "$BOZO_PRIVATE_KEY" \
	--create "$create_code" \
	--json \
		| jq -r .contractAddress
