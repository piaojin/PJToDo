extern crate diesel;

use diesel::prelude::*;

use pal::pj_to_do_pal::PJToDoPal;

lazy_static! {
    pub static ref StaticPJDBConnectionUtil: PJDBConnectionUtil = {
        PJDBConnectionUtil::new()
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