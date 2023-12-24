use std::io::prelude::*;
use std::fs::File;
use std::ffi::CStr;
use std::fs;
use libc::c_char;

pub struct PJFileManager;

impl PJFileManager {
    pub fn create_folder(folder_path: String) {
        match fs::metadata(&folder_path) {
            Ok(_) => {}
            Err(_) => {
                let _ = fs::create_dir(&folder_path);
            }
        }
    }

    fn create_file(path: String) -> std::io::Result<()> {
        let buffer_result = File::create(path.clone());
        match buffer_result {
            Ok(_) => Ok(()),
            Err(e) => {
                pj_error!(
                    "❌❌❌❌❌❌create file error: {:}, {:}❌❌❌❌❌❌",
                    e,
                    path
                );
                Err(e)
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
                let _ = file.read_to_string(&mut file_content);
            }
            Err(e) => {
                pj_error!("read_file_content open file error: {:?}", e);
            }
        }
        file_content
    }

    pub fn wirte_to_file(file_path: String, string: String) -> std::io::Result<()> {
        PJFileManager::wirte_bytes_to_file(file_path, string.as_bytes())
    }

    pub fn wirte_bytes_to_file(file_path: String, bytes: &[u8]) -> std::io::Result<()> {
        let buffer_result = File::create(file_path);
        match buffer_result {
            Ok(mut buffer) => {
                buffer.write_all(bytes)?;
                Ok(())
            }
            Err(e) => {
                pj_error!("❌❌❌❌❌❌create file error: {:}❌❌❌❌❌❌", e);
                Err(e)
            }
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn pj_create_folder(folder_path: *const c_char) {
    assert!(folder_path != std::ptr::null());
    let folder_path = CStr::from_ptr(folder_path).to_string_lossy().into_owned();
    PJFileManager::create_folder(folder_path);
}

#[no_mangle]
pub unsafe extern "C" fn pj_remove_folder(folder_path: *const c_char, all: bool) {
    assert!(folder_path != std::ptr::null());
    let folder_path = CStr::from_ptr(folder_path).to_string_lossy().into_owned();
    let _ = PJFileManager::remove_folder(folder_path, all);
}

#[no_mangle]
pub unsafe extern "C" fn pj_remove_file(file_path: *const c_char) {
    assert!(file_path != std::ptr::null());
    let file_path = CStr::from_ptr(file_path).to_string_lossy().into_owned();
    let _ = PJFileManager::remove_file(file_path);
}
