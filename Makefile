all:
	cargo +nightly build --target=wasm32-unknown-unknown --release
	wasm-gc ./target/wasm32-unknown-unknown/release/rust-wasm-canvas.wasm ./rust.wasm

test:
	cargo +nightly test

check:
	cargo +nightly check
