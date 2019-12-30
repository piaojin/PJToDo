#![feature(rustc_private)]
extern crate cc;

use std::process::Command;

fn main() {
    cc::Build::new().file("/Users/zoey.weng/Desktop/PJToDo/FFI Demo/RustToSwift/RustToSwift/PAL/PJToDoCoreLibPAL_C.c").compile("PJToDoCoreLibPAL");
}
