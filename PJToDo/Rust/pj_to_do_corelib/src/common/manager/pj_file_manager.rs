use std::io::prelude::*;
use std::fs::File;
use pal::pj_to_do_pal::PJToDoPal;
#[allow(unused_imports)]
use common::pj_logger::PJLogger;
use delegates::to_do_file_delegate::{IPJToDoFileDelegate, IPJToDoFileDelegateWrapper};
use delegates::to_do_type_file_delegate::{IPJToDoTypeFileDelegate, IPJToDoTypeFileDelegateWrapper};
use delegates::to_do_tag_file_delegate::{IPJToDoTagFileDelegate, IPJToDoTagFileDelegateWrapper};
use delegates::to_do_settings_file_delegate::{IPJToDoSettingsFileDelegate, IPJToDoSettingsFileDelegateWrapper};
use std::thread;
use service::service_impl::to_do_type_service_impl::{createPJToDoTypeServiceImpl};
use service::service_impl::to_do_service_impl::{createPJToDoServiceImpl};
use service::service_impl::to_do_tag_service_impl::{createPJToDoTagServiceImpl};
use service::service_impl::to_do_settings_service_impl::{createPJToDoSettingsServiceImpl};
use service::to_do_type_service::PJToDoTypeService;
use service::to_do_tag_service::PJToDoTagService;
use service::to_do_settings_service::PJToDoSettingsService;
use service::to_do_service::PJToDoService;
use std::ffi::{CStr, CString};
use std::fs;
use libc::{c_char};

pub struct PJFileManager;

impl PJFileManager {

    pub fn init_folder(folder_path: String) {
        match fs::metadata(&folder_path) {
            Ok(_) => {},
            Err(e) => {
                let _ = fs::create_dir(&folder_path);
            }
        }
    }

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

    pub fn remove_folder(folder_path: String, all: bool) -> std::io::Result<()> {
        if all {
            fs::remove_dir_all(&folder_path)?;
            Ok(())
        } else {
            fs::remove_dir(&folder_path)?;
            Ok(())
        }
    }

    pub fn remove_file(file_path: String) -> std::io::Result<()> {
        fs::remove_file(&file_path)?;
        Ok(())
    }

    pub fn read_file_content<'a>(file_path: &'a str) -> String {
        let mut file_content = String::new();
        match File::open(file_path) {
            Ok(mut file) => {
                file.read_to_string(&mut file_content);
            },
            Err(e) => {
                pj_error!("read_file_content open file error: {:?}", e);
            }
        }
        file_content
    }

    pub fn wirte_to_file(file_path: String, string: String) -> std::io::Result<()> {
        unsafe {
            PJFileManager::wirte_bytes_to_file(file_path, string.as_bytes())
        }
    }

    pub fn wirte_bytes_to_file(file_path: String, bytes: &[u8]) -> std::io::Result<()> {
        unsafe {
            let buffer_result = File::create(file_path);
            match buffer_result {
                Ok(mut buffer) => {
                    buffer.write_all(bytes)?;
                    Ok(())
                },
                Err(e) => {
                    pj_error!("❌create file error: {:}❌", e);
                    Err(e)
                }
            }
        }
    }

    fn wirte_db_type_to_sql_file(delegate: IPJToDoTypeFileDelegate) {
        let i_delegate = IPJToDoTypeFileDelegateWrapper(delegate);
        thread::spawn(move || {
            let todo_type_service = createPJToDoTypeServiceImpl();
            let fetch_type_result = todo_type_service.fetch_data();
            match fetch_type_result {
                Ok(todo_types) => {
                    let mut is_success = true;
                    // Serialize it to a JSON string.
                    let mut content_string: String = String::new();
                    for todo_type in todo_types {
                        let json_string_result = serde_json::to_string(&todo_type);
                        match json_string_result {
                            Ok(type_json_string) => {
                                pj_info!("👉👉: type_json_string: {:?}", type_json_string);
                                content_string.push_str(&format!("{}\n", type_json_string));
                            }
                            Err(e) => {
                                is_success = false;
                                pj_error!("❌❌: wirte_db_type_to_sql_file error: {:?}", e);
                            }
                        }
                    }

                    unsafe {
                        let write_result = PJFileManager::wirte_to_file(PJToDoPal::get_db_type_sql_file_path().to_string(), content_string);
                        match write_result {
                            Ok(_) => {

                            },
                            Err(e) => {
                                is_success = false;
                                pj_error!("❌❌: wirte_db_type_to_sql_file error: {:?}", e);
                            }
                        }
                    }

                    (i_delegate.action_result)(i_delegate.user, is_success);
                },
                Err(e) => {
                    pj_error!("❌❌: wirte_db_type_to_sql_file error: {:?}", e);
                    (i_delegate.action_result)(i_delegate.user, false);
                }
            }
        });
    }

    fn wirte_db_tag_to_sql_file(delegate: IPJToDoTagFileDelegate) {
        let i_delegate = IPJToDoTagFileDelegateWrapper(delegate);
        thread::spawn(move || {
            let todo_tag_service = createPJToDoTagServiceImpl();
            let fetch_tag_result = todo_tag_service.fetch_data();
            match fetch_tag_result {
                Ok(todo_tags) => {
                    let mut is_success = true;
                    // Serialize it to a JSON string.
                    let mut content_string: String = String::new();
                    for todo_tag in todo_tags {
                        let json_string_result = serde_json::to_string(&todo_tag);
                        match json_string_result {
                            Ok(tag_json_string) => {
                                pj_info!("👉👉: tag_json_string: {:?}", tag_json_string);
                                content_string.push_str(&format!("{}\n", tag_json_string));
                            }
                            Err(e) => {
                                is_success = false;
                                pj_error!("❌❌: wirte_db_type_to_sql_file error: {:?}", e);
                            }
                        }
                    }

                    unsafe {
                        let write_result = PJFileManager::wirte_to_file(PJToDoPal::get_db_tag_sql_file_path().to_string(), content_string);
                        match write_result {
                            Ok(_) => {

                            },
                            Err(e) => {
                                is_success = false;
                                pj_error!("❌❌: wirte_db_type_to_sql_file error: {:?}", e);
                            }
                        }
                    }

                    (i_delegate.action_result)(i_delegate.user, is_success);
                },
                Err(e) => {
                    pj_error!("❌❌: wirte_db_type_to_sql_file error: {:?}", e);
                    (i_delegate.action_result)(i_delegate.user, false);
                }
            }
        });
    }

    fn wirte_db_todo_to_sql_file(delegate: IPJToDoFileDelegate) {
        let i_delegate = IPJToDoFileDelegateWrapper(delegate);
        thread::spawn(move || {
            let todo_service = createPJToDoServiceImpl();
            let fetch_type_result = todo_service.fetch_data();
            match fetch_type_result {
                Ok(todos) => {
                    let mut is_success = true;
                    // Serialize it to a JSON string.
                    let mut content_string: String = String::new();
                    for todo in todos {
                        let json_string_result = serde_json::to_string(&todo);
                        match json_string_result {
                            Ok(todo_json_string) => {
                                pj_info!("👉👉: todo_json_string: {:?}", todo_json_string);
                                content_string.push_str(&format!("{}\n", todo_json_string));
                            }
                            Err(e) => {
                                is_success = false;
                                pj_error!("❌❌: wirte_db_todo_to_sql_file error: {:?}", e);
                            }
                        }
                    }

                    unsafe {
                        let write_result = PJFileManager::wirte_to_file(PJToDoPal::get_db_todo_sql_file_path().to_string(), content_string);
                        match write_result {
                            Ok(_) => {

                            },
                            Err(e) => {
                                is_success = false;
                                pj_error!("❌❌: wirte_db_todo_to_sql_file error: {:?}", e);
                            }
                        }
                    }

                    (i_delegate.action_result)(i_delegate.user, is_success);
                },
                Err(e) => {
                    pj_error!("❌❌: wirte_db_todo_to_sql_file error: {:?}", e);
                    (i_delegate.action_result)(i_delegate.user, false);
                }
            }
        });
    }

    fn wirte_db_settings_to_sql_file(delegate: IPJToDoSettingsFileDelegate) {
        let i_delegate = IPJToDoSettingsFileDelegateWrapper(delegate);
        thread::spawn(move || {
            let todo_settings_service = createPJToDoSettingsServiceImpl();
            let fetch_settings_result = todo_settings_service.fetch_data();
            match fetch_settings_result {
                Ok(todo_settings) => {
                    let mut is_success = true;
                    // Serialize it to a JSON string.
                    let mut content_string: String = String::new();
                    for todo_setting in todo_settings {
                        let json_string_result = serde_json::to_string(&todo_setting);
                        match json_string_result {
                            Ok(setting_json_string) => {
                                pj_info!("👉👉: setting_json_string: {:?}", setting_json_string);
                                content_string.push_str(&format!("{}\n", setting_json_string));
                            }
                            Err(e) => {
                                is_success = false;
                                pj_error!("❌❌: wirte_db_type_to_sql_file error: {:?}", e);
                            }
                        }
                    }

                    unsafe {
                        let write_result = PJFileManager::wirte_to_file(PJToDoPal::get_db_todo_settings_sql_file_path().to_string(), content_string);
                        match write_result {
                            Ok(_) => {

                            },
                            Err(e) => {
                                is_success = false;
                                pj_error!("❌❌: wirte_db_type_to_sql_file error: {:?}", e);
                            }
                        }
                    }

                    (i_delegate.action_result)(i_delegate.user, is_success);
                },
                Err(e) => {
                    pj_error!("❌❌: wirte_db_type_to_sql_file error: {:?}", e);
                    (i_delegate.action_result)(i_delegate.user, false);
                }
            }
        });
    }
}

#[no_mangle]
pub unsafe extern "C" fn initFolder(folder_path: *const c_char) {
    assert!(folder_path != std::ptr::null());
    let folder_path = CStr::from_ptr(folder_path).to_string_lossy().into_owned();
    PJFileManager::init_folder(folder_path);
}

#[no_mangle]
pub unsafe extern "C" fn removeFolder(folder_path: *const c_char, all: bool) {
    assert!(folder_path != std::ptr::null());
    let folder_path = CStr::from_ptr(folder_path).to_string_lossy().into_owned();
    PJFileManager::remove_folder(folder_path, all);
}

#[no_mangle]
pub unsafe extern "C" fn removeFile(file_path: *const c_char) {
    assert!(file_path != std::ptr::null());
    let file_path = CStr::from_ptr(file_path).to_string_lossy().into_owned();
    PJFileManager::remove_file(file_path);
}

#[no_mangle]
pub unsafe extern "C" fn wirteDBTypeToSQLFile(delegate: IPJToDoTypeFileDelegate) {
    PJFileManager::wirte_db_type_to_sql_file(delegate);
}

#[no_mangle]
pub unsafe extern "C" fn wirteDBTagToSQLFile(delegate: IPJToDoTagFileDelegate) {
    PJFileManager::wirte_db_tag_to_sql_file(delegate);
}

#[no_mangle]
pub unsafe extern "C" fn wirteDBToDoToSQLFile(delegate: IPJToDoFileDelegate) {
    PJFileManager::wirte_db_todo_to_sql_file(delegate);
}

#[no_mangle]
pub unsafe extern "C" fn wirteDBSettingsToSQLFile(delegate: IPJToDoSettingsFileDelegate) {
    PJFileManager::wirte_db_settings_to_sql_file(delegate);
}
