use std::ffi::{CStr};
use common::pj_logger::PJLogger;
use c_binding_extern::c_binding_extern::*;
use db::pj_db_connection_util::pj_db_connection_util::*;

pub struct PJToDoPal;

impl PJToDoPal {
    pub fn sqlite_url<'a>() -> &'a str {
        // from pal
        &SQLiteUrl
    }

    pub fn db_gzip_path<'a>() -> &'a str {
        // from pal
        &DBGZipPath
    }

    pub fn db_uncompress_path<'a>() -> &'a str {
        &DBUnCompressPath
    }

    pub unsafe fn say_hi_from_rust() {
        println!("PJToDoPal is ready in Rust CoreLib!");
        test_pal_from_Swift();
        let get_db_path = PJToDoPal::sqlite_url(); //unsafe
        pj_info!("get_db_path: {:}", get_db_path);
    }
}

#[no_mangle]
pub unsafe extern "C" fn test_pal_from_rust() {
    PJToDoPal::say_hi_from_rust();
}
