# !/bin/sh

echo "******Rust RustCoreLib RustCargoScript Clean Script Start******"

echo "RUST_LIB_DIR_NAME: $RUST_LIB_DIR_NAME"

pwd
cd ..

current_path=`pwd`
rust_lib_path="$current_path/Rust/$RUST_LIB_DIR_NAME"

cd $rust_lib_path
echo "rust_lib_path: $rust_lib_path"
pwd

root_path=~
cargo_path="$root_path/.cargo/bin/cargo"
echo "cargo_path: $cargo_path"
alias cargo='$cargo_path'
echo "******cargo clean:"
cargo clean

cd $SRCROOT

echo "******Rust RustCoreLib RustCargoScript Clean Script End******"

