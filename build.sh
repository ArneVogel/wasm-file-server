#!/bin/sh

git submodule update --init --recursive

# build wasmtime
cd wasmtime
cargo build --release
cd ..

# build the file-server
cd file-server
make
cd ..
