use delegates::to_do_type_delegate::{IPJToDoTypeDelegate, IPJToDoTypeDelegateWrapper};
use service::to_do_type_service::{PJToDoTypeService, insert_to_do_type, delete_to_do_type, update_to_do_type, find_to_do_type_by_id, find_to_do_type_by_name};
use service::service_impl::to_do_type_service_impl::{createPJToDoTypeServiceImpl};
use view_models::to_do_type_view_model::PJToDoTypeViewModel;
use to_do_type::to_do_type::{ToDoTypeInsert, createToDoTypeInsert, ToDoType, createToDoType};
use common::{free_rust_object, free_rust_any_object};
use common::pj_logger::PJLogger;
use std::ffi::{CString, CStr};
use libc::{c_char};
use std::sync::{Arc, Mutex};
use std::thread;
use std::marker::{Send, Sync};
use std::ops::Deref;

/*
* cbindgen didn't support Box<dyn PJToDoTypeService> type,so I need to use PJToDoTypeServiceController to define Box<dyn PJToDoTypeService>.
*/
#[repr(C)]
pub struct PJToDoTypeServiceController {
    todo_type_service: Box<dyn PJToDoTypeService>,
}

impl PJToDoTypeServiceController {
    fn new() -> PJToDoTypeServiceController {
        let serviceController = PJToDoTypeServiceController {
            todo_type_service: Box::new(createPJToDoTypeServiceImpl()),
        };
        serviceController
    }
}

impl Drop for PJToDoTypeServiceController {
    fn drop(&mut self) {
        println!("PJToDoTypeServiceController -> drop");
    }
}

unsafe impl Send for PJToDoTypeServiceController {}
unsafe impl Sync for PJToDoTypeServiceController {}

#[repr(C)]
pub struct PJToDoTypeController {
    pub delegate: IPJToDoTypeDelegate,
    pub todo_typ_service_controller: *mut PJToDoTypeServiceController,
    pub todo_type_insert: *mut ToDoTypeInsert,
    pub todo_type: *mut ToDoType,
}

unsafe impl Send for PJToDoTypeController {}
unsafe impl Sync for PJToDoTypeController {}

impl PJToDoTypeController {
    fn new(delegate: IPJToDoTypeDelegate) -> PJToDoTypeController {
        let c_str_insert = CString::new("".to_string()).unwrap();
        let c_str_type = CString::new("".to_string()).unwrap();
        let controller = unsafe {
            PJToDoTypeController {
                delegate: delegate,
                todo_typ_service_controller: Box::into_raw(Box::new(
                    PJToDoTypeServiceController::new(),
                )),
                todo_type_insert: createToDoTypeInsert(c_str_insert.into_raw()),
                todo_type: createToDoType(c_str_type.into_raw()),
            }
        };
        controller
    }

    /**
     * 添加分类
     */
    pub unsafe fn insert_to_do_type(&self, to_do_type: *const ToDoTypeInsert) {
        pj_info!("insert_to_do_type: {}", (*to_do_type).type_name);
        assert!(!to_do_type.is_null());
        let i_delegate = IPJToDoTypeDelegateWrapper((&self.delegate) as *const IPJToDoTypeDelegate);

        let result = insert_to_do_type(
            &(&(*self.todo_typ_service_controller)).todo_type_service,
            &(*to_do_type),
        );

        match result {
            Ok(_) => {
                (i_delegate.insert_result)(i_delegate.user, true);
                pj_info!("insert to_do_type success");
            }
            Err(e) => {
                (i_delegate.insert_result)(i_delegate.user, false);
                pj_error!("insert to_do_type fiald: {}", e);
            }
        }
    }

    pub unsafe fn delete_to_do_type(&self, to_do_type_id: i32) {
        pj_info!("delete_to_do_type to_do_type_id: {}", to_do_type_id);

        let i_delegate = IPJToDoTypeDelegateWrapper((&self.delegate) as *const IPJToDoTypeDelegate);

        let result = delete_to_do_type(
            &(&(*self.todo_typ_service_controller)).todo_type_service,
            to_do_type_id,
        );

        match result {
            Ok(_) => {
                (i_delegate.delete_result)(i_delegate.user, true);
                pj_info!("delete to_do_type success");
            }
            Err(e) => {
                (i_delegate.delete_result)(i_delegate.user, false);
                pj_error!("delete to_do_type fiald: {}", e);
            }
        }
    }

    pub unsafe fn update_to_do_type(&self, to_do_type: *const ToDoType) {
        assert!(!to_do_type.is_null());

        let i_delegate = IPJToDoTypeDelegateWrapper((&self.delegate) as *const IPJToDoTypeDelegate);

        let result = update_to_do_type(
            &(&(*self.todo_typ_service_controller)).todo_type_service,
            &(*to_do_type),
        );

        match result {
            Ok(_) => {
                (i_delegate.update_result)(i_delegate.user, true);
                pj_info!("update to_do_type success");
            }
            Err(e) => {
                (i_delegate.update_result)(i_delegate.user, false);
                pj_error!("update to_do_type fiald: {}", e);
            }
        }
    }

    pub unsafe fn find_to_do_type_by_id(&mut self, to_do_type_id: i32) {
        pj_info!("find_to_do_type_by_id to_do_type_id: {}", to_do_type_id);

        let i_delegate = IPJToDoTypeDelegateWrapper((&self.delegate) as *const IPJToDoTypeDelegate);

        let result = find_to_do_type_by_id(
            &(&(*self.todo_typ_service_controller)).todo_type_service,
            to_do_type_id,
        );

        match result {
            Ok(to_do_type) => {
                /*free the old todoType before set the new one*/
                free_rust_any_object(self.todo_type);
                let to_do_type_ptr = Box::into_raw(Box::new(to_do_type));
                self.todo_type = to_do_type_ptr;
                (i_delegate.find_byId_result)(i_delegate.user, to_do_type_ptr, true);
                pj_info!("delete to_do_type success");
            }
            Err(e) => {
                let mut to_do_type = ToDoType {
                    id: -1,
                    type_name: "".to_owned(),
                };
                let to_do_type_ptr = &mut to_do_type as *mut ToDoType;
                (i_delegate.find_byId_result)(i_delegate.user, to_do_type_ptr, false);
                pj_error!("delete to_do_type fiald: {}", e);
            }
        }
    }

    pub unsafe fn find_to_do_type_by_name(&mut self, type_name: String) {
        pj_info!("find_to_do_type_by_name type_name: {}", type_name);

        let i_delegate = IPJToDoTypeDelegateWrapper((&self.delegate) as *const IPJToDoTypeDelegate);

        let result = find_to_do_type_by_name(
            &(&(*self.todo_typ_service_controller)).todo_type_service,
            type_name,
        );

        match result {
            Ok(to_do_type) => {
                /*free the old todoType before set the new one*/
                free_rust_any_object(self.todo_type);
                let to_do_type_ptr = Box::into_raw(Box::new(to_do_type));
                self.todo_type = to_do_type_ptr;
                (i_delegate.find_byName_result)(i_delegate.user, to_do_type_ptr, true);
                pj_info!("find_to_do_type_by_name success");
            }
            Err(e) => {
                let mut to_do_type = ToDoType {
                    id: -1,
                    type_name: "".to_owned(),
                };
                let to_do_type_ptr = &mut to_do_type as *mut ToDoType;
                (i_delegate.find_byName_result)(i_delegate.user, to_do_type_ptr, false);
                pj_error!("find_to_do_type_by_name fiald: {}", e);
            }
        }
    }
}

impl Drop for PJToDoTypeController {
    fn drop(&mut self) {
        //PJToDoTypeController被释放，告诉当前持有PJToDoTypeDelegate对象的所有权者做相应的处理
        unsafe {
            free_rust_any_object(self.todo_typ_service_controller);
            free_rust_any_object(self.todo_type_insert);
            free_rust_any_object(self.todo_type);
        }
        println!("PJToDoTypeController -> drop");
    }
}

/*** extern "C" ***/

#[no_mangle]
pub extern "C" fn createPJToDoTypeController(
    delegate: IPJToDoTypeDelegate,
) -> *mut PJToDoTypeController {
    let controller = PJToDoTypeController::new(delegate);
    Box::into_raw(Box::new(controller))
}

#[no_mangle]
pub unsafe extern "C" fn insertToDoType(
    ptr: *mut PJToDoTypeController,
    toDoType: *const ToDoTypeInsert,
) {
    if ptr.is_null() || toDoType.is_null() {
        pj_error!("ptr or toDoType: *mut PJToDoTypeController is null!");
        return;
    }

    let controler = &mut *ptr;
    let toDoType = &*toDoType;

    thread::spawn(move || {
        println!("insertToDoType thread::spawn");
        controler.insert_to_do_type(toDoType);
    });
}

#[no_mangle]
pub unsafe extern "C" fn deleteToDoType(ptr: *mut PJToDoTypeController, toDoTypeId: i32) {
    if ptr.is_null() {
        pj_error!("ptr: *mut PJToDoTypeController is null!");
        return;
    }

    let controler = &mut *ptr;

    thread::spawn(move || {
        println!("insertToDoType thread::spawn");
        controler.delete_to_do_type(toDoTypeId);
    });
}

#[no_mangle]
pub unsafe extern "C" fn updateToDoType(ptr: *mut PJToDoTypeController, toDoType: *const ToDoType) {
    if ptr.is_null() || toDoType.is_null() {
        pj_error!("ptr or toDoType: *mut PJToDoTypeController is null!");
        return;
    }

    let controler = &mut *ptr;
    let toDoType = &*toDoType;

    thread::spawn(move || {
        println!("insertToDoType thread::spawn");
        controler.update_to_do_type(toDoType);
    });
}

#[no_mangle]
pub unsafe extern "C" fn findToDoType(ptr: *mut PJToDoTypeController, toDoTypeId: i32) {
    if ptr.is_null() {
        pj_error!("ptr: *mut PJToDoTypeController is null!");
        return;
    }

    let controler = &mut *ptr;

    thread::spawn(move || {
        println!("insertToDoType thread::spawn");
        controler.find_to_do_type_by_id(toDoTypeId);
    });
}

#[no_mangle]
pub unsafe extern "C" fn findToDoTypeByName(
    ptr: *mut PJToDoTypeController,
    type_name: *const c_char,
) {
    if ptr.is_null() || type_name.is_null() {
        pj_error!("ptr or typeName: *mut PJToDoTypeController is null!");
        return;
    }

    let controler = &mut *ptr;
    let type_name = CStr::from_ptr(type_name).to_string_lossy().into_owned();

    thread::spawn(move || {
        println!("insertToDoType thread::spawn");
        controler.find_to_do_type_by_name(type_name);
    });
}

#[no_mangle]
pub unsafe extern "C" fn getToDoType(ptr: *const PJToDoTypeController) -> *const ToDoTypeInsert {
    assert!(!ptr.is_null());
    let controler = &*ptr;
    controler.todo_type_insert
}

#[no_mangle]
pub unsafe extern "C" fn free_rust_PJToDoTypeController(ptr: *mut PJToDoTypeController) {
    if ptr.is_null() {
        return;
    };
    Box::from_raw(ptr); //unsafe
}
