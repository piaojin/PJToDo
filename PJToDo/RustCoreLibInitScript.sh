#!/bin/sh

echo "******Rust RustCoreLib RustCoreLibInitScript Start******"

current_path=`pwd`
rust_lib_path="Rust/pj_to_do_corelib"
cd $rust_lib_path
pwd

lib_name="libpj_to_do_corelib.a"
lib_path="$current_path/iOS/RustBindings/$lib_name"

root_path=~
cargo_path="$root_path/.cargo/bin/cargo"
echo "cargo_path: $cargo_path"
alias cargo='$cargo_path'

if [[ "$CONFIGURATION" == "Release" ]]
then

    echo "******cargo lipo --release:"
    cargo lipo --release
    cp $current_path/$rust_lib_path/target/universal/release/$lib_name $lib_path

else

    echo "******cargo lipo --debug:"
    cargo lipo --verbose
    # copy staticlib to iOS 
    cp $current_path/$rust_lib_path/target/universal/debug/$lib_name $lib_path

fi

# cargo lipo --verbose

# cp $current_path/$rust_lib_path/target/universal/debug/libpj_to_do_corelib.a $current_path/iOS/RustBindings/libpj_to_do_corelib.a

cd $current_path

echo "******Rust RustCoreLib RustCoreLibInitScript End******"