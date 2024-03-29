#[macro_use]
pub mod pj_logger;

pub mod request_config;

pub mod pj_serialize;

pub mod utils;

pub mod manager;

extern crate rustc_serialize;

use libc::{c_void, c_char};

//析构对象
#[no_mangle]
pub unsafe extern "C" fn pj_free_rust_object(ptr: *mut c_void) {
    if ptr != std::ptr::null_mut() {
        // Box::from_raw(ptr); //unsafe
        free_rust_any_object(ptr);
    };
}

//析构对象
#[no_mangle]
pub unsafe extern "C" fn pj_free_rust_string(ptr: *mut c_char) {
    if ptr != std::ptr::null_mut() {
        free_rust_any_object(ptr);
    };
}

pub unsafe fn free_rust_any_object<T>(pointer: *mut T)
where
    T: std::any::Any,
{
    let is_null = pointer.is_null();
    pj_info!("free_rust_any_object is_null: {}", is_null);
    if !is_null {
        let _ = Box::from_raw(pointer); //unsafe
    };
}
