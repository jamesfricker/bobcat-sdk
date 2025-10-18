
RELEASE := target/wasm32-unknown-unknown/release

BUILT := ${RELEASE}/chainlink-vrf-test.wasm

PROJECTS := chainlink-vrf-test

all: chainlink-vrf-test.wasm

${BUILT}: $(shell find ${PROJECTS} Cargo.toml -type f)
	@cargo build --target wasm32-unknown-unknown --release

chainlink-vrf-test.wasm: ${BUILT}
	@./wasm-post.sh ${RELEASE}/chainlink-vrf-test.wasm chainlink-vrf-test.wasm
