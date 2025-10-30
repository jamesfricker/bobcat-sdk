
RELEASE := target/wasm32-unknown-unknown/release

BUILT := ${RELEASE}/chainlink-vrf-test.wasm

PROJECTS := \
	chainlink-vrf-test \
	transient-exchange \
	console-log-txt \
	counter \
	mul-div \
	beacon \
	panic-revert

all: wasm

.PHONY: wasm clean

${BUILT}: $(shell find ${PROJECTS} Cargo.toml -type f)
	@cargo build --target wasm32-unknown-unknown --release

wasm: \
	chainlink-vrf-test.wasm \
	transient-exchange.wasm \
	console-log-txt.wasm \
	counter.wasm \
	mul-div.wasm \
	beacon.wasm \
	panic-revert.wasm

chainlink-vrf-test.wasm: ${BUILT}
	@rm -f chainlink-vrf-test.wasm
	@./wasm-post.sh ${RELEASE}/chainlink-vrf-test.wasm chainlink-vrf-test.wasm

transient-exchange.wasm: ${BUILT}
	@rm -f transient-exchange.wasm
	@./wasm-post.sh ${RELEASE}/transient-exchange.wasm transient-exchange.wasm

console-log-txt.wasm: ${BUILT}
	@rm -f console-log-txt.wasm
	@./wasm-post.sh ${RELEASE}/console-log-txt.wasm console-log-txt.wasm

counter.wasm: ${BUILT}
	@rm -f counter.wasm
	@./wasm-post.sh ${RELEASE}/libcounter.wasm counter.wasm

mul-div.wasm: ${BUILT}
	@rm -f mul-div.wasm
	@./wasm-post.sh ${RELEASE}/mul-div.wasm mul-div.wasm

beacon.wasm: ${BUILT}
	@rm -f beacon.wasm
	@./wasm-post.sh ${RELEASE}/beacon.wasm beacon.wasm

panic-revert.wasm: ${BUILT}
	@rm -f panic-revert.wasm
	@./wasm-post.sh ${RELEASE}/panic-revert.wasm panic-revert.wasm

clean:
	@rm -rf \
		target \
		chainlink-vrf-test.wasm \
		transient-exchange.wasm \
		console-log-txt.wasm \
		counter.wasm \
		mul-div.wasm \
		beacon.wasm \
		panic-revert.wasm
