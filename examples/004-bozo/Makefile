
bozo.wasm: $(shell find Cargo.* src -type f)
	@rm -f bozo.wasm
	@cargo build --release --target wasm32-unknown-unknown
	@./wasm-post.sh \
		../target/wasm32-unknown-unknown/release/bozo.wasm \
		bozo.wasm
	@./check-codesize.sh bozo.wasm
