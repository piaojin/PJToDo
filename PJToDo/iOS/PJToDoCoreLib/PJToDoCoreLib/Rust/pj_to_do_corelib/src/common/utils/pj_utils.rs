extern crate hyper;
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use std::mem;
use libc::c_char;
use std::ffi::{CString, CStr};
use crate::common::rustc_serialize::base64::{STANDARD, ToBase64, FromBase64};

pub struct PJUtils;

impl PJUtils {
    pub fn string_to_static_str(s: String) -> &'static str {
        unsafe {
            let ret = mem::transmute(&s as &str);
            mem::forget(s);
            ret
        }
    }

    pub fn create_cstring_from(s: &str) -> CString {
        let cstring_res: Result<CString, std::ffi::NulError> = CString::new(s); //unsafe
        let mut cstring: CString =
            unsafe { CString::from_vec_with_nul_unchecked(b"default\0".to_vec()) };
        match cstring_res {
            Ok(value) => {
                cstring = value;
            }

            Err(e) => {
                pj_warn!("Create CString from {} failed: {}", s, e);
            }
        }
        cstring
    }
}

#[no_mangle]
pub unsafe extern "C" fn pj_convert_str_to_base64str(ptr: *const c_char) -> *mut c_char {
    assert!(ptr != std::ptr::null());
    let original_string = CStr::from_ptr(ptr).to_string_lossy().into_owned();
    let converted_str = PJUtils::create_cstring_from(&original_string);
    let config = STANDARD;
    let converted_base64_string = converted_str.as_bytes().to_base64(config);
    let converted_base64_cstring = PJUtils::create_cstring_from(&converted_base64_string);
    converted_base64_cstring.into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn pj_convert_base64str_to_str(ptr: *const c_char) -> *mut c_char {
    assert!(ptr != std::ptr::null());
    let base64_string = CStr::from_ptr(ptr).to_string_lossy().into_owned();
    let temp_c_char: *mut c_char = std::ptr::null_mut();
    let res = base64_string.from_base64();
    if res.is_ok() {
        match res {
            Ok(s) => {
                let opt_bytes_res = String::from_utf8(s);
                match opt_bytes_res {
                    Ok(opt_bytes) => {
                        let c_str = PJUtils::create_cstring_from(&opt_bytes);
                        c_str.into_raw()
                    }

                    Err(_) => temp_c_char,
                }
            }

            Err(_) => temp_c_char,
        }
    } else {
        temp_c_char
    }
}
