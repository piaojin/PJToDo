#[macro_use]
pub mod pj_logger;

pub mod request_config;

pub mod pj_serialize;

pub mod pj_utils;

extern crate rustc_serialize;

use libc::{c_void, c_char};
use std::ffi::{CString, CStr};
use common::rustc_serialize::base64::{STANDARD, ToBase64};

//析构对象
#[no_mangle]
pub unsafe extern "C" fn ConvertStrToBase64Str(ptr: *const c_char) -> *mut c_char {
    assert!(ptr != std::ptr::null());
    let original_string = CStr::from_ptr(ptr).to_string_lossy().into_owned();
    let converted_str = CString::new(original_string).unwrap(); //unsafe
    let config = STANDARD;
    let converted_base64_string = converted_str.as_bytes().to_base64(config);
    let converted_base64_cstring = CString::new(converted_base64_string).unwrap();
    converted_base64_cstring.into_raw()
}

//析构对象
#[no_mangle]
pub unsafe extern "C" fn free_rust_object(ptr: *mut c_void) {
    if ptr != std::ptr::null_mut() {
        // Box::from_raw(ptr); //unsafe
        free_rust_any_object(ptr);
    };
}

//析构对象
#[no_mangle]
pub unsafe extern "C" fn free_rust_string(ptr: *mut c_char) {
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
        Box::from_raw(pointer); //unsafe
    };
}
