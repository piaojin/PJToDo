#!/bin/sh

echo "👉👉👉👉👉👉Rust Custom Build Script Start👈👈👈👈👈👈"

echo "👉👉👉👉👉👉ACTION: $ACTION"
echo "👉👉👉👉👉👉Current Shell: $SHELL"
echo "👉👉👉👉👉👉BuildConfiguration=$CONFIGURATION"
echo "👉👉👉👉👉👉RUST_BUILD_BINDINGS_DIR path: $RUST_BUILD_BINDINGS_DIR"
echo "👉👉👉👉👉👉RUST_BUILD_BINDINGS_HEAD_PATH path: $RUST_BUILD_BINDINGS_HEAD_PATH"
echo "👉👉👉👉👉👉RUST_PAL_BUILD_BINDINGS_HEAD_PATH path: $RUST_PAL_BUILD_BINDINGS_HEAD_PATH"

root_path=~
cargo_path="$root_path/.cargo/bin/cargo"
echo "👉👉👉👉👉👉cargo_path: $cargo_path"
alias cargo='$cargo_path'
echo "👉👉👉👉👉👉cargo lipo --version:"
cargo lipo --version

pwd
cd ..

rust_lib_path="${SRCROOT}/${TARGET_NAME}/Rust/$RUST_LIB_DIR_NAME"

######bindgen######
bindgen_path="$root_path/.cargo/bin/bindgen"
echo "👉👉👉👉👉👉bindgen_path: $bindgen_path"
alias bindgen='$bindgen_path'

echo "👉👉👉👉👉👉bindgen $RUST_PAL_BUILD_BINDINGS_HEAD_PATH -o "$rust_lib_path/src/app_pal_bindings.rs"
:"
# bindgen $RUST_PAL_BUILD_BINDINGS_HEAD_PATH -o "$rust_lib_path/src/app_pal_bindings.rs" --verbose --raw-line "#[link(name = "PJToDoCoreLibPAL")]" --rust-target "nightly" --rustfmt-bindings --objc-extern-crate

bindgen $RUST_PAL_BUILD_BINDINGS_HEAD_PATH -o "$rust_lib_path/src/c_binding_extern/c_binding_extern.rs" --raw-line "#[link(name = \"PJToDoCoreLibPAL\")]" --no-rustfmt-bindings

echo "👉👉👉👉👉👉rust_lib_path: $rust_lib_path"
cd $rust_lib_path
pwd

rustup_path="$root_path/.cargo/bin/rustup"
alias rustup='$rustup_path'

######important: reset evn of shell nor cbindgen will get error######
unset IPHONEOS_DEPLOYMENT_TARGET
unset $(env | grep '^SDK' | cut -d'=' -f1)

######important: cargo build######
if [[ "$CONFIGURATION" == "Debug" ]]
then
    echo "🙏🙏🙏🙏🙏🙏 Run cargo lipo:"
    cargo lipo --verbose --targets=aarch64-apple-ios,x86_64-apple-ios
    # cargo lipo --verbose --targets=aarch64-apple-ios,x86_64-apple-ios
    # cargo build --verbose
    # LLVM_SYS_70_PREFIX=/path/to/llvm cargo build

    # copy staticlib to iOS 
    # echo "******cp $rust_lib_path/target/universal/debug/$RUST_LIB_NAME $RUST_BUILD_BINDINGS_DIR:"
    # cp $rust_lib_path/target/universal/debug/$RUST_LIB_NAME $RUST_BUILD_BINDINGS_DIR
else
    echo "🙏🙏🙏🙏🙏🙏Run cargo lipo --release:"
    cargo lipo --release -vv --targets=aarch64-apple-ios,armv7s-apple-ios
    # cargo lipo --release -vv --targets=aarch64-apple-ios,armv7s-apple-ios
    # cp $rust_lib_path/target/universal/debug/$RUST_LIB_NAME $RUST_BUILD_BINDINGS_DIR
fi

######format Rust code######
cargo +nightly fmt

######cbindgen binding head file for Swift######
cbindgen_path="$root_path/.cargo/bin/cbindgen"
echo "👉👉👉👉👉👉cbindgen_path: $cbindgen_path"
alias cbindgen='$cbindgen_path'

rustup_path="$root_path/.cargo/bin/rustup"
alias rustup='$rustup_path'
# rustup target add aarch64-apple-ios armv7-apple-ios armv7s-apple-ios x86_64-apple-ios i386-apple-ios
echo "🙏🙏🙏🙏🙏🙏rustup run nightly cbindgen $rust_lib_path --lockfile Cargo.lock --crate $RUST_LIB_DIR_NAME -o $RUST_BUILD_BINDINGS_HEAD_PATH
:"
# stable | nightly
rustup run nightly cbindgen $rust_lib_path --lockfile Cargo.lock --crate $RUST_LIB_DIR_NAME -o $RUST_BUILD_BINDINGS_HEAD_PATH

cd $SRCROOT

echo "👉👉👉👉👉👉Rust Custom Build Script End👈👈👈👈👈👈"
