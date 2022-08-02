# wasm-file-server

Proof of concept web server hosted as WebAssembly module.

More info: https://www.arnevogel.com/wasm-hosted

## Build
You will need a recent Rust installation (1.62.1 tested) with the wasm32-wasi toolchain installed (`rustup target add wasm32-wasi`).

To build simply run `./build.sh` in the root directory. This will initialize the wasmtime submodule, apply the patch to it, and build it as well as the file server.

To run the server, after building, change into the `file-server` directory and run `make run`.
