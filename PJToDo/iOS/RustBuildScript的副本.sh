#!/bin/bash

echo "******Rust Custom Build Script Start******"
echo "RUST_BUILD_BINDINGS_DIR path: $RUST_BUILD_BINDINGS_DIR"
echo "RUST_BUILD_BINDINGS_HEAD_PATH path: $RUST_BUILD_BINDINGS_HEAD_PATH"
# cargo_path=`which   cargo`
cargo_path="/Users/zoey.weng/.cargo/bin/cargo"
echo "cargo_path: $cargo_path"
alias cargo="$cargo_path"
cargo   lipo  --version
pwd
cd ..
current_path=`pwd`
rust_lib_path="$current_path/Rust/$RUST_LIB_DIR_NAME"
echo "rust_lib_path: $rust_lib_path"
cd $rust_lib_path
pwd
# cargo   lipo
cbindgen_path="/Users/zoey.weng/.cargo/bin/cbindgen"
echo "cbindgen_path: $cbindgen_path"
alias cbindgen="$cbindgen_path"
cbindgen -o $RUST_BUILD_BINDINGS_HEAD_PATH
echo "******Rust Custom Build Script End******"