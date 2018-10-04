#!/bin/sh

current_path=`pwd`
rust_lib_path="Rust/pj_to_do_corelib"
cd $rust_lib_path
pwd
cargo lipo --verbose

cp $current_path/$rust_lib_path/target/universal/debug/libpj_to_do_corelib.a $current_path/iOS/RustBindings/libpj_to_do_corelib.a

cd $current_path