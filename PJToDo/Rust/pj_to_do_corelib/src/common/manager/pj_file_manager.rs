use std::io::prelude::*;
use std::fs::File;
use pal::pj_to_do_pal::PJToDoPal;
#[allow(unused_imports)]
use common::pj_logger::PJLogger;
use delegates::to_do_file_delegate::{IPJToDoFileDelegate, IPJToDoFileDelegateWrapper};
use delegates::to_do_type_file_delegate::{IPJToDoTypeFileDelegate, IPJToDoTypeFileDelegateWrapper};
use std::thread;
use service::service_impl::to_do_type_service_impl::{createPJToDoTypeServiceImpl};
use service::to_do_type_service::PJToDoTypeService;
use std::ffi::{CString};


pub struct PJFileManager;

impl PJFileManager {
    fn init_db_data_sql_file() {
        unsafe {
            PJFileManager::init_db_sql_file(PJToDoPal::get_db_todo_sql_file_path().to_string());
            PJFileManager::init_db_sql_file(PJToDoPal::get_db_type_sql_file_path().to_string());
            PJFileManager::init_db_sql_file(PJToDoPal::get_db_tag_sql_file_path().to_string());
            PJFileManager::init_db_sql_file(PJToDoPal::get_db_todo_settings_sql_file_path().to_string());
        }
    }

    fn init_db_sql_file(path: String) -> std::io::Result<()> {
        unsafe {
            let buffer_result = File::create(path.clone());
            match buffer_result {
                Ok(_) => {
                    Ok(())
                },
                Err(e) => {
                    pj_error!("❌create db_sql_file error: {:}, {:}❌", e, path);
                    Err(e)
                }
            }
        }
    }

    fn wirte_to_file(file_path: String, string: String) -> std::io::Result<()> {
        unsafe {
            let buffer_result = File::create(file_path);
            match buffer_result {
                Ok(mut buffer) => {
                    buffer.write(string.as_bytes())?;
                    Ok(())
                },
                Err(e) => {
                    pj_error!("❌create db_data_sql_file error: {:}❌", e);
                    Err(e)
                }
            }
        }
    }

    //Result<Vec<ToDoType>, diesel::result::Error>
    fn wirte_db_type_to_sql_file() {
        //delegate: IPJToDoTypeFileDelegate
        // let i_delegate = IPJToDoFileDelegateWrapper(delegate);
        thread::spawn(move || {
            let todo_type_service = createPJToDoTypeServiceImpl();
            let fetch_type_result = todo_type_service.fetch_data();
            match fetch_type_result {
                Ok(todo_types) => {
                    // Serialize it to a JSON string.
                    for todo_type in todo_types {
                        let json_string_result = serde_json::to_string(&todo_type);
                        match json_string_result {
                            Ok(type_json_string) => {
                                // let c_str = CString::new(json_string).unwrap();
                                pj_info!("👉👉: type_json_string: {:?}", type_json_string);
                                unsafe {
                                    PJFileManager::wirte_to_file(PJToDoPal::get_db_type_sql_file_path().to_string(), format!("{}/n", type_json_string));
                                }
                                // (i_delegate.request_result)(i_delegate.user, c_char, true);
                            }
                            Err(e) => {
                                pj_error!("❌❌: wirte_db_type_to_sql_file error: {:?}", e);
                            }
                        }
                    }
                },
                Err(e) => {
                    pj_error!("❌❌: wirte_db_type_to_sql_file error: {:?}", e);
                }
            }
        });
    }

}

#[no_mangle]
pub unsafe extern "C" fn init_db_data_sql_file() {
    PJFileManager::init_db_data_sql_file();
}

#[no_mangle]
pub unsafe extern "C" fn wirteDBTypeToSQLFile() {
    PJFileManager::wirte_db_type_to_sql_file();
}