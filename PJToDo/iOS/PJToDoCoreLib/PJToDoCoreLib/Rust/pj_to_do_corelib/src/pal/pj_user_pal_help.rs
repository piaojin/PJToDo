use c_binding_extern::c_binding_extern::*;
use std::ffi::{CStr};

pub struct PJUserPALHelp;

impl PJUserPALHelp {
    pub unsafe fn get_user_authorization_str() -> String {
        let authorization_str = CStr::from_ptr(get_authorization_str())
            .to_string_lossy()
            .into_owned();
        authorization_str
    }

    pub unsafe fn get_access_token_str() -> String {
        let access_token_str = CStr::from_ptr(get_access_token_str())
            .to_string_lossy()
            .into_owned();
        access_token_str
    }
}
