extern crate diesel;

use std::ffi::{CStr};
use diesel::prelude::*;

use super::pj_database::Database;

use pal::pj_to_do_pal::PJToDoPal;
use c_binding_extern::c_binding_extern::{get_db_path};

use db::tables::schema::{Table_ToDoType, Table_ToDoTag, Table_ToDo, Table_ToDoType_Create_Sql, Table_ToDoTag_Create_Sql, Table_ToDo_Create_Sql};

use std::collections::HashMap;

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
    pub connection: SqliteConnection,
    pub data_base: Database,
    pub data_base_url: String,
}

unsafe impl Sync for PJDBConnectionUtil {}

impl PJDBConnectionUtil {

    pub fn new() -> PJDBConnectionUtil {
        PJDBConnectionUtil {
            connection: PJDBConnectionUtil::establish_connection(PJToDoPal::sqlite_url()),
            data_base: Database::new(PJToDoPal::sqlite_url()),
            data_base_url: PJToDoPal::sqlite_url().into(),
        }
    }

    //sqlite_url is const and get from pal(UI)
    pub fn establish_connection(sqlite_url: &str) -> SqliteConnection {
        SqliteConnection::establish(&sqlite_url)
            .unwrap_or_else(|e| panic!("Error connecting to {}, error: {}", sqlite_url, e))
    }

    pub fn init_database(&self) {
        if !self.data_base.exists() {
            self.data_base.create();
        }
    }

    pub fn init_tables(&self) {
        let mut tables = HashMap::new();
        tables.insert(Table_ToDoType.to_string(), Table_ToDoType_Create_Sql.to_string());
        tables.insert(Table_ToDoTag.to_string(), Table_ToDoTag_Create_Sql.to_string());
        tables.insert(Table_ToDo.to_string(), Table_ToDo_Create_Sql.to_string());
        
        for (table, table_create_sql) in tables {
            if !self.data_base.table_exists(&table) {
                self.data_base.execute(&table_create_sql);
            }
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn init_database() {
    StaticPJDBConnectionUtil.init_database();
}

#[no_mangle]
pub unsafe extern "C" fn init_tables() {
    StaticPJDBConnectionUtil.init_tables();
}