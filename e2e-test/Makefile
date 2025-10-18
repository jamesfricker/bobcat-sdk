
RELEASE := target/wasm32-unknown-unknown/release

BUILT := ${RELEASE}/chainlink-vrf-test.wasm

PROJECTS := chainlink-vrf-test transient-exchange

all: wasm

.PHONY: wasm

${BUILT}: $(shell find ${PROJECTS} Cargo.toml -type f)
	@cargo build --target wasm32-unknown-unknown --release

wasm: chainlink-vrf-test.wasm transient-exchange.wasm

chainlink-vrf-test.wasm: ${BUILT}
	@./wasm-post.sh ${RELEASE}/chainlink-vrf-test.wasm chainlink-vrf-test.wasm

transient-exchange.wasm: ${BUILT}
	@./wasm-post.sh ${RELEASE}/transient-exchange.wasm transient-exchange.wasm
