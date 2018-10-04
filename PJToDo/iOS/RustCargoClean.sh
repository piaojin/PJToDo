#!/bin/sh

echo "******Rust Custom Clean Script Start******"

echo "RustMake_RUST_LIB_DIR_NAME: $RustMake_RUST_LIB_DIR_NAME"

pwd
cd ..

current_path=`pwd`
rust_lib_path="$current_path/Rust/pj_to_do_corelib"

cd $rust_lib_path
echo "rust_lib_path: $rust_lib_path"
pwd

root_path=~
cargo_path="$root_path/.cargo/bin/cargo"
echo "cargo_path: $cargo_path"
alias cargo='$cargo_path'
echo "******cargo clean:"
cargo clean

echo "******Rust Custom Clean Script End******"