extern crate diesel;

use std::ffi::{CStr};
use diesel::prelude::*;

use pal::pj_to_do_pal::PJToDoPal;
use c_binding_extern::c_binding_extern::{get_db_path};

lazy_static! {
    pub static ref StaticPJDBConnectionUtil: PJDBConnectionUtil = {
        PJDBConnectionUtil::new()
    };

    pub static ref SQLiteUrl: String = {
        let get_db_path = unsafe {
            CStr::from_ptr(get_db_path()).to_string_lossy().into_owned()
        };
        get_db_path
    };
}

pub struct PJDBConnectionUtil {
    pub connection: SqliteConnection
}

unsafe impl Sync for PJDBConnectionUtil {}

impl PJDBConnectionUtil {

    pub fn new() -> PJDBConnectionUtil {
        PJDBConnectionUtil {
            connection: PJDBConnectionUtil::establish_connection(PJToDoPal::sqlite_url())
        }
    }

    //sqlite_url is const and get from pal(UI)
    pub fn establish_connection(sqlite_url: &str) -> SqliteConnection {
        SqliteConnection::establish(&sqlite_url)
            .unwrap_or_else(|e| panic!("Error connecting to {}, error: {}", sqlite_url, e))
    }
}