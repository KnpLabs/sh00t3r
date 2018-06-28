.PHONY: build
build:
	cargo +nightly build --target wasm32-unknown-unknown --release
	# wasm-gc target/wasm32-unknown-unknown/release/sh00t3r.wasm -o target/wasm32-unknown-unknown/release/sh00t3r.wasm
	wasm-bindgen target/wasm32-unknown-unknown/release/sh00t3r.wasm --out-dir public/
