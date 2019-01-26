extern crate hyper;
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use common::pj_serialize::PJSerdeDeserialize;
use std::mem;
use libc::{c_char};
use std::ffi::{CString, CStr};
use common::rustc_serialize::base64::{STANDARD, ToBase64};

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

    pub fn parse_data<'a, T>(body: &'a hyper::Chunk) -> Result<T, serde_json::Error>
    where
        T: std::fmt::Debug + PJSerdeDeserialize<'a>,
    {
        // try to parse as json with serde_json
        let parse_result = serde_json::from_slice(body);
        pj_info!("parse data result: {:?}", parse_result);
        parse_result
    }
}

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
