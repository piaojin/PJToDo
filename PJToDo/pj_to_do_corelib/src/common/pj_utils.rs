use std::mem;

pub struct PJUtils;

impl  PJUtils {
    pub fn string_to_static_str(s: String) -> &'static str {
        unsafe {
            let ret = mem::transmute(&s as &str);
            mem::forget(s);
            ret
        }
    }
}