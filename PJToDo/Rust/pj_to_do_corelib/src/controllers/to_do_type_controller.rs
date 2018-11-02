use delegates::to_do_type_delegate::{IPJToDoTypeDelegate, IPJToDoTypeDelegateWrapper};
use service::to_do_type_service::{PJToDoTypeService, insert_to_do_type, delete_to_do_type, update_to_do_type, find_to_do_type_by_id, find_to_do_type_by_name, fetch_data};
use service::service_impl::to_do_type_service_impl::{createPJToDoTypeServiceImpl};
use to_do_type::to_do_type::{ToDoTypeInsert, ToDoType, createToDoType};
use common::{free_rust_any_object};
#[allow(unused_imports)]
use common::pj_logger::PJLogger;
use std::ffi::{CString, CStr};
use libc::{c_char};
use std::thread;
use std::marker::{Send, Sync};

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
    pub find_result_todo_type: *mut ToDoType,
    pub todo_types: *mut Vec<ToDoType>,
}

unsafe impl Send for PJToDoTypeController {}
unsafe impl Sync for PJToDoTypeController {}

impl PJToDoTypeController {
    fn new(delegate: IPJToDoTypeDelegate) -> PJToDoTypeController {
        let c_str_type = CString::new("".to_string()).unwrap();
        let controller = unsafe {
            PJToDoTypeController {
                delegate: delegate,
                todo_typ_service_controller: Box::into_raw(Box::new(
                    PJToDoTypeServiceController::new(),
                )),
                find_result_todo_type: createToDoType(c_str_type.into_raw()),
                todo_types: Box::into_raw(Box::new(Vec::new())),
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
            }
            Err(_e) => {
                (i_delegate.insert_result)(i_delegate.user, false);
            }
        }
    }

    pub unsafe fn delete_to_do_type(&self, to_do_type_id: i32) {
        let i_delegate = IPJToDoTypeDelegateWrapper((&self.delegate) as *const IPJToDoTypeDelegate);

        let result = delete_to_do_type(
            &(&(*self.todo_typ_service_controller)).todo_type_service,
            to_do_type_id,
        );

        match result {
            Ok(_) => {
                (i_delegate.delete_result)(i_delegate.user, true);
            }
            Err(_e) => {
                (i_delegate.delete_result)(i_delegate.user, false);
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
            }
            Err(_e) => {
                (i_delegate.update_result)(i_delegate.user, false);
            }
        }
    }

    pub unsafe fn find_to_do_type_by_id(&mut self, to_do_type_id: i32) {
        let i_delegate = IPJToDoTypeDelegateWrapper((&self.delegate) as *const IPJToDoTypeDelegate);

        let result = find_to_do_type_by_id(
            &(&(*self.todo_typ_service_controller)).todo_type_service,
            to_do_type_id,
        );

        match result {
            Ok(to_do_type) => {
                /*free the old todoType before set the new one*/
                free_rust_any_object(self.find_result_todo_type);
                let to_do_type_ptr = Box::into_raw(Box::new(to_do_type));
                self.find_result_todo_type = to_do_type_ptr;
                (i_delegate.find_byId_result)(i_delegate.user, to_do_type_ptr, true);
            }
            Err(_e) => {
                let mut to_do_type = ToDoType {
                    id: -1,
                    type_name: "".to_owned(),
                };
                let to_do_type_ptr = &mut to_do_type as *mut ToDoType;
                (i_delegate.find_byId_result)(i_delegate.user, to_do_type_ptr, false);
            }
        }
    }

    pub unsafe fn find_to_do_type_by_name(&mut self, type_name: String) {
        let i_delegate = IPJToDoTypeDelegateWrapper((&self.delegate) as *const IPJToDoTypeDelegate);

        let result = find_to_do_type_by_name(
            &(&(*self.todo_typ_service_controller)).todo_type_service,
            type_name,
        );

        match result {
            Ok(to_do_type) => {
                /*free the old todoType before set the new one*/
                free_rust_any_object(self.find_result_todo_type);
                let to_do_type_ptr = Box::into_raw(Box::new(to_do_type));
                self.find_result_todo_type = to_do_type_ptr;
                (i_delegate.find_byName_result)(i_delegate.user, to_do_type_ptr, true);
            }
            Err(_e) => {
                let mut to_do_type = ToDoType {
                    id: -1,
                    type_name: "".to_owned(),
                };
                let to_do_type_ptr = &mut to_do_type as *mut ToDoType;
                (i_delegate.find_byName_result)(i_delegate.user, to_do_type_ptr, false);
            }
        }
    }

    pub unsafe fn fetch_data(&mut self) {
        let i_delegate = IPJToDoTypeDelegateWrapper((&self.delegate) as *const IPJToDoTypeDelegate);

        let result = fetch_data(&(&(*self.todo_typ_service_controller)).todo_type_service);

        match result {
            Ok(to_do_types) => {
                /*free the old todo_types before set the new one*/
                free_rust_any_object(self.todo_types);
                self.todo_types = Box::into_raw(Box::new(to_do_types));
                (i_delegate.fetch_data_result)(i_delegate.user, true);
            }
            Err(_e) => {
                (i_delegate.fetch_data_result)(i_delegate.user, false);
            }
        }
    }

    pub unsafe fn todo_type_at_index(&self, index: i32) -> *const ToDoType {
        let index: usize = index as usize;
        assert!(index <= self.get_count());
        let todo_type: *const ToDoType = &((*(self.todo_types))[index]);
        todo_type
    }

    pub unsafe fn get_count(&self) -> usize {
        if self.todo_types.is_null() {
            ()
        }

        let count = (*(self.todo_types)).len();
        count
    }
}

impl Drop for PJToDoTypeController {
    fn drop(&mut self) {
        //PJToDoTypeController被释放，告诉当前持有PJToDoTypeDelegate对象的所有权者做相应的处理
        unsafe {
            free_rust_any_object(self.todo_typ_service_controller);
            free_rust_any_object(self.find_result_todo_type);
            free_rust_any_object(self.todo_types);
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
        pj_error!("ptr or toDoType: *mut insertToDoType is null!");
        assert!(!ptr.is_null() && !toDoType.is_null());
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
        pj_error!("ptr: *mut deleteToDoType is null!");
        assert!(!ptr.is_null());
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
        pj_error!("ptr or toDoType: *mut updateToDoType is null!");
        assert!(!ptr.is_null() && !toDoType.is_null());
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
        pj_error!("ptr: *mut findToDoType is null!");
        assert!(!ptr.is_null());
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
        pj_error!("ptr or typeName: *mut findToDoTypeByName is null!");
        assert!(!ptr.is_null() && !type_name.is_null());
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
pub unsafe extern "C" fn fetchToDoTypeData(ptr: *mut PJToDoTypeController) {
    if ptr.is_null() {
        pj_error!("ptr or toDoType: *mut fetchData is null!");
        assert!(!ptr.is_null());
    }
    let controler = &mut *ptr;
    controler.fetch_data()
}

#[no_mangle]
pub unsafe extern "C" fn todoTypeAtIndex(
    ptr: *const PJToDoTypeController,
    index: i32,
) -> *const ToDoType {
    if ptr.is_null() {
        pj_error!("ptr or toDoType: *mut todoTypeAtIndex is null!");
        assert!(!ptr.is_null());
    }
    let controler = &*ptr;
    controler.todo_type_at_index(index)
}

#[no_mangle]
pub unsafe extern "C" fn getToDoTypeCount(ptr: *const PJToDoTypeController) -> i32 {
    if ptr.is_null() {
        pj_error!("ptr or toDoType: *mut getToDoTypeCount is null!");
        assert!(!ptr.is_null());
    }
    let controler = &*ptr;
    controler.get_count() as i32
}

#[no_mangle]
pub unsafe extern "C" fn free_rust_PJToDoTypeController(ptr: *mut PJToDoTypeController) {
    if ptr.is_null() {
        return;
    };
    Box::from_raw(ptr); //unsafe
}
