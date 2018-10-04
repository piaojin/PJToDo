#!/bin/sh

echo "******Rust Custom Build Script Start******"

echo "ACTION: $ACTION"
echo "Current Shell: $SHELL"
echo "BuildConfiguration=$CONFIGURATION"
echo "RUST_BUILD_BINDINGS_DIR path: $RUST_BUILD_BINDINGS_DIR"
echo "RUST_BUILD_BINDINGS_HEAD_PATH path: $RUST_BUILD_BINDINGS_HEAD_PATH"

root_path=~
cargo_path="$root_path/.cargo/bin/cargo"
echo "cargo_path: $cargo_path"
alias cargo='$cargo_path'
echo "******cargo lipo --version:"
cargo lipo --version

pwd
cd ..

current_path=`pwd`
rust_lib_path="$current_path/Rust/$RUST_LIB_DIR_NAME"

echo "rust_lib_path: $rust_lib_path"
cd $rust_lib_path
pwd

rustup_path="$root_path/.cargo/bin/rustup"
alias rustup='$rustup_path'

if [[ "$CONFIGURATION" == "Debug" ]]
then
    echo "******cargo lipo:"
    cargo lipo --verbose
    # cargo build --verbose
    # LLVM_SYS_70_PREFIX=/path/to/llvm cargo build

    # copy staticlib to iOS 
    echo "******cp $rust_lib_path/target/universal/debug/$RUST_LIB_NAME $RUST_BUILD_BINDINGS_DIR:"
    cp $rust_lib_path/target/universal/debug/$RUST_LIB_NAME $RUST_BUILD_BINDINGS_DIR
else
    echo "******cargo lipo --release:"
    cargo lipo --release

    cp $rust_lib_path/target/universal/debug/$RUST_LIB_NAME $RUST_BUILD_BINDINGS_DIR
fi

cbindgen_path="$root_path/.cargo/bin/cbindgen"
echo "cbindgen_path: $cbindgen_path"
alias cbindgen='$cbindgen_path'

rustup_path="$root_path/.cargo/bin/rustup"
alias rustup='$rustup_path'
# rustup target add aarch64-apple-ios armv7-apple-ios armv7s-apple-ios x86_64-apple-ios i386-apple-ios
echo "******rustup run nightly cbindgen $rust_lib_path --lockfile Cargo.lock --crate $RUST_LIB_DIR_NAME -o $RUST_BUILD_BINDINGS_HEAD_PATH
:"
# stable | nightly
rustup run nightly cbindgen $rust_lib_path --lockfile Cargo.lock --crate $RUST_LIB_DIR_NAME -o $RUST_BUILD_BINDINGS_HEAD_PATH

cd $SRCROOT

echo "******Rust Custom Build Script End******"