#!/bin/sh

git submodule update --init --recursive

# build wasmtime
cd wasmtime
git apply ../socket.patch
cargo build --release
cd ..

# build the file-server
cd file-server
make
cd ..
