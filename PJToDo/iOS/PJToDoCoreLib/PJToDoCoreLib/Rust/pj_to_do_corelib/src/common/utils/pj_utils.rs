extern crate hyper;
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use std::mem;
use libc::{c_char};
use std::ffi::{CString, CStr};
use common::rustc_serialize::base64::{STANDARD, ToBase64, FromBase64};

pub struct PJUtils;

impl PJUtils {
    pub fn string_to_static_str(s: String) -> &'static str {
        unsafe {
            let ret = mem::transmute(&s as &str);
            mem::forget(s);
            ret
        }
    }
}

pub struct PJHttpUtils;

impl PJHttpUtils {
    /*body to String*/
    pub fn hyper_body_to_string(body: hyper::Chunk) -> String {
        let v = body.to_vec();
        String::from_utf8_lossy(&v).to_string()
    }
}

#[no_mangle]
pub unsafe extern "C" fn pj_convert_str_to_base64str(ptr: *const c_char) -> *mut c_char {
    assert!(ptr != std::ptr::null());
    let original_string = CStr::from_ptr(ptr).to_string_lossy().into_owned();
    let converted_str = CString::new(original_string).unwrap(); //unsafe
    let config = STANDARD;
    let converted_base64_string = converted_str.as_bytes().to_base64(config);
    let converted_base64_cstring = CString::new(converted_base64_string).unwrap();
    converted_base64_cstring.into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn pj_convert_base64str_to_str(ptr: *const c_char) -> *mut c_char {
    assert!(ptr != std::ptr::null());
    let base64_string = CStr::from_ptr(ptr).to_string_lossy().into_owned();
    let temp_c_char: *mut c_char = std::ptr::null_mut();
    let res = base64_string.from_base64();
    if res.is_ok() {
        let opt_bytes = String::from_utf8(res.unwrap());
        if opt_bytes.is_ok() {
            let c_str = CString::new(opt_bytes.unwrap()).unwrap();
            c_str.into_raw()
        } else {
            temp_c_char
        }
    } else {
        temp_c_char
    }
}
