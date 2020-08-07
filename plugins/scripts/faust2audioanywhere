#!/bin/bash

ASSETS_PATH=/Users/br-gaster/dev/wasm/audioplugin_wasm/plugins/assets
FAUST=/Users/br-gaster/dev/wasm/faust_bgaster/build/bin/faust

if [ $# -eq 0 ]; then
    echo "No arguments provided"
    exit 1
fi

cat $ASSETS_PATH/faust_rust_prefix.rs
$FAUST -lang rust $1
cat $ASSETS_PATH/faust_rust_postfix.rs
