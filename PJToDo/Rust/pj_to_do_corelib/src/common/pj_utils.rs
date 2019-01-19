extern crate hyper;
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use common::pj_serialize::PJSerdeDeserialize;
use std::mem;

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
