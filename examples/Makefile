
.PHONY: clean

target/wasm32-unknown-unknown/release/bobcat_sdk_examples_001.wasm: \
	$(shell find Cargo.* 001-counter 002-mul_div 003-camelot-swapping -type f)
	@cargo build --target wasm32-unknown-unknown --release

clean:
	@rm -rf target
