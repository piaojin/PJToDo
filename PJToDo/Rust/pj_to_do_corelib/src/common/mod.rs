#[macro_use]
pub mod pj_logger;

pub mod request_config;

pub mod pj_serialize;

pub mod pj_utils;

use libc::{c_void};

//析构对象
#[no_mangle]
pub unsafe extern "C" fn free_rust_object(ptr: *mut c_void) {
    if !ptr.is_null() {
        // Box::from_raw(ptr); //unsafe
        free_rust_any_object(ptr);
    };
}

pub unsafe fn free_rust_any_object(ptr: *mut std::any::Any) {
    if !ptr.is_null() {
        Box::from_raw(ptr); //unsafe
    };
}
