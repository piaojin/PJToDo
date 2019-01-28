use std::io::prelude::*;
use std::fs::File;
use pal::pj_to_do_pal::PJToDoPal;
#[allow(unused_imports)]
use common::pj_logger::PJLogger;
use delegates::to_do_file_delegate::{IPJToDoFileDelegate, IPJToDoFileDelegateWrapper};
use std::thread;
use service::service_impl::to_do_type_service_impl::{createPJToDoTypeServiceImpl};

pub struct PJFileManager;

impl PJFileManager {
    fn init_db_data_sql_file() -> std::io::Result<()> {
        unsafe {
            let buffer_result = File::create(PJToDoPal::get_db_data_sql_file_path());
            match buffer_result {
                Ok(_) => {
                    Ok(())
                },
                Err(e) => {
                    pj_error!("❌create db_data_sql_file error: {:}❌", e);
                    Err(e)
                }
            }
        }
    }

    fn wirte_to_file(file_path: String, string: String) -> std::io::Result<()> {
        unsafe {
            let buffer_result = File::create(PJToDoPal::get_db_data_sql_file_path());
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

    fn wirte_db_data_to_sql_file(delegate: IPJToDoFileDelegate) {
        let i_delegate = IPJToDoFileDelegateWrapper(delegate);
        thread::spawn(move || {
            let todo_type_service = createPJToDoTypeServiceImpl();
            let fetch_type_result = to_do_type_service.fetch_data();
        });
    }
}

#[no_mangle]
pub unsafe extern "C" fn init_db_data_sql_file() {
    PJFileManager::init_db_data_sql_file();
}