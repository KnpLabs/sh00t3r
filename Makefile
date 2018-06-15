.PHONY: build
build:
	cargo +nightly build --target wasm32-unknown-unknown --release
	wasm-gc target/wasm32-unknown-unknown/release/sh00t3r.wasm -o public/sh00t3r.gc.wasm
