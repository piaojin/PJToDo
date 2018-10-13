use libc::{c_char};

#[link(name = "PJToDoCoreLibPAL")]
// #[link(name="hello", kind="static")]
extern "C" { 
    pub fn test_pal_from_Swift(); 

    pub fn get_db_path() -> *const c_char;
}