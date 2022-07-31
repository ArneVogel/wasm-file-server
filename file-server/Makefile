all:
	RUSTFLAGS="--cfg tokio_unstable" cargo build --target=wasm32-wasi

run:
	../wasmtime/target/release/wasmtime run --dir=public --tcplisten 127.0.0.1:4000 --env FD_COUNT=3 target/wasm32-wasi/debug/$(shell basename $(shell pwd)).wasm
