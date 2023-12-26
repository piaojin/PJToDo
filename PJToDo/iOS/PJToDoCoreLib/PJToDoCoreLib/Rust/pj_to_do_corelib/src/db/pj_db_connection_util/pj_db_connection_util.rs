extern crate diesel;

use std::ffi::CStr;
use diesel::prelude::*;

use super::pj_database::Database;

use crate::pal::pj_to_do_pal::PJToDoPal;
use crate::c_binding_extern::c_binding_extern::*;

use crate::db::tables::schema::{
    Table_ToDoType, Table_ToDoTag, Table_ToDo, Table_ToDoSettings, Table_ToDoType_Create_Sql,
    Table_ToDoTag_Create_Sql, Table_ToDo_Create_Sql, Table_ToDoSettings_Create_Sql,
};

use std::collections::HashMap;

use std::sync::{Arc, Mutex};

lazy_static! {
    pub static ref StaticPJDBConnectionUtil: Arc<Mutex<PJDBConnectionUtil>> =
        Arc::new(Mutex::new(PJDBConnectionUtil::new()));
    pub static ref SQLiteUrl: String = {
        let get_db_path = unsafe { CStr::from_ptr(get_db_path()).to_string_lossy().into_owned() };
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

    pub fn update_connection(&mut self, sqlite_url: &str) {
        self.connection = PJDBConnectionUtil::establish_connection(sqlite_url);
    }

    pub fn init_database(&self) {
        if !self.data_base.exists() {
            self.data_base.create();
        }
    }

    pub fn init_tables(&self) {
        let mut tables = HashMap::new();
        tables.insert(
            Table_ToDoType.to_string(),
            Table_ToDoType_Create_Sql.to_string(),
        );

        tables.insert(
            Table_ToDoTag.to_string(),
            Table_ToDoTag_Create_Sql.to_string(),
        );

        tables.insert(
            Table_ToDoSettings.to_string(),
            Table_ToDoSettings_Create_Sql.to_string(),
        );

        tables.insert(Table_ToDo.to_string(), Table_ToDo_Create_Sql.to_string());

        for (table, table_create_sql) in tables {
            if !self.data_base.table_exists(&table) {
                self.data_base.execute(&table_create_sql);
            }
        }
    }

    pub fn fetch_connection<F, E, T>(
        completion_handler: F,
        failure_handler: E,
    ) -> Result<T, diesel::result::Error>
    where
        F: Fn(std::sync::MutexGuard<PJDBConnectionUtil>) -> T,
        E: Fn(diesel::result::Error) -> diesel::result::Error,
    {
        match StaticPJDBConnectionUtil.lock() {
            Ok(value) => Ok(completion_handler(value)),

            Err(e) => {
                pj_error!("fetchData failure {}", e);
                Err(failure_handler(
                    diesel::result::Error::BrokenTransactionManager,
                ))
            }
        }
    }

    pub fn get_connection() -> Option<std::sync::MutexGuard<'static, PJDBConnectionUtil>> {
        match StaticPJDBConnectionUtil.lock() {
            Ok(value) => Some(value),

            Err(e) => {
                pj_error!("fetchData failure {}", e);
                None
            }
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn pj_update_db_connection() {
    match StaticPJDBConnectionUtil.lock() {
        Ok(mut value) => {
            value.update_connection(PJToDoPal::sqlite_url());
        }

        Err(e) => {
            pj_error!("update db connection failed: {}", e);
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn pj_init_data_base() {
    match StaticPJDBConnectionUtil.lock() {
        Ok(value) => {
            value.init_database();
        }

        Err(e) => {
            pj_error!("init database failed: {}", e);
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn pj_init_tables() {
    match StaticPJDBConnectionUtil.lock() {
        Ok(value) => {
            value.init_tables();
        }

        Err(e) => {
            pj_error!("init tables failed: {}", e);
        }
    }
}
