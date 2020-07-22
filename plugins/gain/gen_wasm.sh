#!/bin/bash

cargo build --release --target wasm32-unknown-unknown

if [ $? -eq 0 ]
then
    echo Optimizing WASM output into pkg/gain.wasm
    wasm-opt -O4 target/wasm32-unknown-unknown/release/gain.wasm -o pkg/gain.wasm
else 
    echo Compile failed
fi