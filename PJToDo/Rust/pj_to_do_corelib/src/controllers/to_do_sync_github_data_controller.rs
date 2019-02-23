use delegates::to_do_sync_github_data_delegate::{IPJToDoSyncGitHubDataDelegate, IPJToDoSyncGitHubDataDelegateWrapper};
use service::to_do_type_service::{PJToDoTypeService};
use service::to_do_tag_service::{PJToDoTagService};
use service::service_impl::to_do_type_service_impl::{createPJToDoTypeServiceImpl};
use to_do_type::to_do_type::{ToDoTypeInsert};
use service::service_impl::to_do_tag_service_impl::{createPJToDoTagServiceImpl};
use to_do_tag::to_do_tag::{ToDoTagInsert};
use service::to_do_service::{PJToDoService};
use service::service_impl::to_do_service_impl::{createPJToDoServiceImpl};
use to_do::to_do::{ToDoInsert};
use mine::todo_settings::{ToDoSettingsInsert};
use service::to_do_settings_service::{PJToDoSettingsService};
use service::service_impl::to_do_settings_service_impl::{createPJToDoSettingsServiceImpl};
use common::{free_rust_any_object};
#[allow(unused_imports)]
use common::pj_logger::PJLogger;
use std::ffi::{CString, CStr};
use libc::{c_char};
use std::thread;
use std::marker::{Send, Sync};
use common::pj_serialize::{PJSerializeUtils};

/*The create and free are both in Rust. only free PJToDoSyncGitHubDataController in Swift.*/
#[repr(C)]
pub struct PJToDoSyncGitHubDataController {
    pub delegate: IPJToDoSyncGitHubDataDelegate,
}

unsafe impl Send for PJToDoSyncGitHubDataController {}
unsafe impl Sync for PJToDoSyncGitHubDataController {}

impl PJToDoSyncGitHubDataController {
    fn new(delegate: IPJToDoSyncGitHubDataDelegate) -> PJToDoSyncGitHubDataController {
        PJToDoSyncGitHubDataController {
            delegate: delegate,
        }
    }

    pub unsafe fn sync_type_data_result(&mut self, file_path: String) {
        let result_vec = PJSerializeUtils::from_file_by_line::<ToDoTypeInsert>(&file_path);
        let i_delegate = IPJToDoSyncGitHubDataDelegateWrapper((&self.delegate) as *const IPJToDoSyncGitHubDataDelegate);
        let result_vec_len = result_vec.len();
        if result_vec_len > 0 {
            let mut sync_success_count = 0;
            let service: Box<dyn PJToDoTypeService> = Box::new(createPJToDoTypeServiceImpl());
            for result in result_vec {
                match result {
                    Ok(model) => {
                        let insert_result = service.insert_todo_type(&model);
                        match insert_result {
                            Ok(_) => {
                                sync_success_count += 1;
                            },
                            Err(e) => {
                                pj_error!("sync_type_data_result error: {:?}", e);
                            }
                        }
                    },
                    Err(e) => {
                        pj_error!("sync_type_data_result error: {:?}", e);
                    }
                }
            }
            (i_delegate.sync_type_data_result)(i_delegate.user, sync_success_count == result_vec_len);
        } else {
            (i_delegate.sync_type_data_result)(i_delegate.user, true);
        }
    }

    pub unsafe fn sync_tag_data_result(&mut self, file_path: String) {
        let result_vec = PJSerializeUtils::from_file_by_line::<ToDoTagInsert>(&file_path);
        let i_delegate = IPJToDoSyncGitHubDataDelegateWrapper((&self.delegate) as *const IPJToDoSyncGitHubDataDelegate);
        let result_vec_len = result_vec.len();
        if result_vec_len > 0 {
            let mut sync_success_count = 0;
            let service: Box<dyn PJToDoTagService> = Box::new(createPJToDoTagServiceImpl());
            for result in result_vec {
                match result {
                    Ok(model) => {
                        let insert_result = service.insert_todo_tag(&model);
                        match insert_result {
                            Ok(_) => {
                                sync_success_count += 1;
                            },
                            Err(e) => {
                                pj_error!("sync_tag_data_result error: {:?}", e);
                            }
                        }
                    },
                    Err(e) => {
                        pj_error!("sync_tag_data_result error: {:?}", e);
                    }
                }
            }
            (i_delegate.sync_type_data_result)(i_delegate.user, sync_success_count == result_vec_len);
        } else {
            (i_delegate.sync_type_data_result)(i_delegate.user, true);
        }
    }

    pub unsafe fn sync_settings_data_result(&mut self, file_path: String) {
        let result_vec = PJSerializeUtils::from_file_by_line::<ToDoSettingsInsert>(&file_path);
        let i_delegate = IPJToDoSyncGitHubDataDelegateWrapper((&self.delegate) as *const IPJToDoSyncGitHubDataDelegate);
        let result_vec_len = result_vec.len();
        if result_vec_len > 0 {
            let mut sync_success_count = 0;
            let service: Box<dyn PJToDoSettingsService> = Box::new(createPJToDoSettingsServiceImpl());
            for result in result_vec {
                match result {
                    Ok(model) => {
                        let insert_result = service.insert_todo_settings(&model);
                        match insert_result {
                            Ok(_) => {
                                sync_success_count += 1;
                            },
                            Err(e) => {
                                pj_error!("sync_settings_data_result error: {:?}", e);
                            }
                        }
                    },
                    Err(e) => {
                        pj_error!("sync_settings_data_result error: {:?}", e);
                    }
                }
            }
            (i_delegate.sync_type_data_result)(i_delegate.user, sync_success_count == result_vec_len);
        } else {
            (i_delegate.sync_type_data_result)(i_delegate.user, true);
        }
    }

    pub unsafe fn sync_todo_data_result(&mut self, file_path: String) {
        let result_vec = PJSerializeUtils::from_file_by_line::<ToDoInsert>(&file_path);
        let i_delegate = IPJToDoSyncGitHubDataDelegateWrapper((&self.delegate) as *const IPJToDoSyncGitHubDataDelegate);
        let result_vec_len = result_vec.len();
        if result_vec_len > 0 {
            let mut sync_success_count = 0;
            let service: Box<dyn PJToDoService> = Box::new(createPJToDoServiceImpl());
            for result in result_vec {
                match result {
                    Ok(model) => {
                        let insert_result = service.insert_todo(&model);
                        match insert_result {
                            Ok(_) => {
                                sync_success_count += 1;
                            },
                            Err(e) => {
                                pj_error!("sync_todo_data_result error: {:?}", e);
                            }
                        }
                    },
                    Err(e) => {
                        pj_error!("sync_todo_data_result error: {:?}", e);
                    }
                }
            }
            (i_delegate.sync_type_data_result)(i_delegate.user, sync_success_count == result_vec_len);
        } else {
            (i_delegate.sync_type_data_result)(i_delegate.user, true);
        }
    }
}

impl Drop for PJToDoSyncGitHubDataController {
    fn drop(&mut self) {
        println!("PJToDoSyncGitHubDataController -> drop");
    }
}

/*** extern "C" ***/

#[no_mangle]
pub extern "C" fn createPJToDoSyncGitHubDataController(
    delegate: IPJToDoSyncGitHubDataDelegate,
) -> *mut PJToDoSyncGitHubDataController {
    let controller = PJToDoSyncGitHubDataController::new(delegate);
    Box::into_raw(Box::new(controller))
}

#[no_mangle]
pub unsafe extern "C" fn syncGitHubTypeData(
    ptr: *mut PJToDoSyncGitHubDataController,
    file_path: *const c_char,
) {
    if ptr == std::ptr::null_mut() || file_path == std::ptr::null_mut() {
        pj_error!("ptr or toDoType: *mut syncGitHubTypeData is null!");
        assert!(ptr != std::ptr::null_mut() && file_path != std::ptr::null_mut());
    }

    let controler = &mut *ptr;
    let file_path = CStr::from_ptr(file_path).to_string_lossy().into_owned();

    thread::spawn(move || {
        println!("syncGitHubTypeData thread::spawn");
        controler.sync_type_data_result(file_path);
    });
}

#[no_mangle]
pub unsafe extern "C" fn syncGitHubTagData(
    ptr: *mut PJToDoSyncGitHubDataController,
    file_path: *const c_char,
) {
    if ptr == std::ptr::null_mut() || file_path == std::ptr::null_mut() {
        pj_error!("ptr or toDoType: *mut syncGitHubTagData is null!");
        assert!(ptr != std::ptr::null_mut() && file_path != std::ptr::null_mut());
    }

    let controler = &mut *ptr;
    let file_path = CStr::from_ptr(file_path).to_string_lossy().into_owned();

    thread::spawn(move || {
        println!("syncGitHubTagData thread::spawn");
        controler.sync_tag_data_result(file_path);
    });
}

#[no_mangle]
pub unsafe extern "C" fn syncGitHubSettingsData(
    ptr: *mut PJToDoSyncGitHubDataController,
    file_path: *const c_char,
) {
    if ptr == std::ptr::null_mut() || file_path == std::ptr::null_mut() {
        pj_error!("ptr or toDoType: *mut syncGitHubSettingsData is null!");
        assert!(ptr != std::ptr::null_mut() && file_path != std::ptr::null_mut());
    }

    let controler = &mut *ptr;
    let file_path = CStr::from_ptr(file_path).to_string_lossy().into_owned();

    thread::spawn(move || {
        println!("syncGitHubSettingsData thread::spawn");
        controler.sync_settings_data_result(file_path);
    });
}

#[no_mangle]
pub unsafe extern "C" fn syncGitHubToDoData(
    ptr: *mut PJToDoSyncGitHubDataController,
    file_path: *const c_char,
) {
    if ptr == std::ptr::null_mut() || file_path == std::ptr::null_mut() {
        pj_error!("ptr or toDoType: *mut syncGitHubToDoData is null!");
        assert!(ptr != std::ptr::null_mut() && file_path != std::ptr::null_mut());
    }

    let controler = &mut *ptr;
    let file_path = CStr::from_ptr(file_path).to_string_lossy().into_owned();

    thread::spawn(move || {
        println!("syncGitHubToDoData thread::spawn");
        controler.sync_todo_data_result(file_path);
    });
}

#[no_mangle]
pub unsafe extern "C" fn free_rust_PJToDoSyncGitHubDataController(ptr: *mut PJToDoSyncGitHubDataController) {
    if ptr != std::ptr::null_mut() {
        Box::from_raw(ptr); //unsafe
    };
}
