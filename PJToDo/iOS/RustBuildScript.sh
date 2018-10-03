#!/bin/sh

echo "******Rust Custom Build Script Start******"
echo "BuildConfiguration=$CONFIGURATION"
echo "RUST_BUILD_BINDINGS_DIR path: $RUST_BUILD_BINDINGS_DIR"
echo "RUST_BUILD_BINDINGS_HEAD_PATH path: $RUST_BUILD_BINDINGS_HEAD_PATH"
# cargo_path=`which   cargo`
# find /Users/zoey.weng/.cargo/bin/ -name "cargo"
root_path=~
cargo_path="$root_path/.cargo/bin/cargo"
echo "cargo_path: $cargo_path"
alias cargo='$cargo_path'
cargo lipo --version
pwd
cd ..
current_path=`pwd`
rust_lib_path="$current_path/Rust/$RUST_LIB_DIR_NAME"
echo "rust_lib_path: $rust_lib_path"
cd $rust_lib_path
pwd

if [[ "$CONFIGURATION" == "Debug" ]]
then
    cargo build
else
    cargo lipo --release
fi

cbindgen_path="$root_path/.cargo/bin/cbindgen"
echo "cbindgen_path: $cbindgen_path"
alias cbindgen='$cbindgen_path'
rustup_path="$root_path/.cargo/bin/rustup"
alias rustup='$rustup_path'
# stable | nightly
rustup run nightly cbindgen $rust_lib_path --lockfile Cargo.lock --crate $RUST_LIB_DIR_NAME -o $RUST_BUILD_BINDINGS_HEAD_PATH
echo "******Rust Custom Build Script End******"