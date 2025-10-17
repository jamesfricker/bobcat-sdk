
built: $(shell find -name 'Cargo.*' -or -name '*.rs')
	@cargo build --target wasm32-unknown-unknown --release
	@touch built
