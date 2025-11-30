
.PHONY: build clean

build: bozo.wasm bootstrap.zip

bozo.wasm: $(shell find Cargo.* src -type f)
	@rm -f bozo.wasm
	@cargo build --release --target wasm32-unknown-unknown
	@./wasm-post.sh \
		target/wasm32-unknown-unknown/release/bobcat-examples-004.wasm \
		bozo.wasm
	@./check-codesize.sh bozo.wasm

bozo.out: $(shell find -name 'go.*' -or -name '*.go' -type f)
	@go build -o bozo.out

bootstrap: bozo.out
	@cp bozo.out bootstrap

bootstrap.zip: bootstrap
	@zip bootstrap.zip bootstrap

clean:
	@rm -rf bootstrap.zip bootstrap bozo.wasm target bozo.out
