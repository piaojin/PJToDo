use delegates::to_do_type_delegate::{IPJToDoTypeDelegate, IPJToDoTypeDelegateWrapper};
use service::to_do_type_service::{PJToDoTypeService, insert_to_do_type};
use service::service_impl::to_do_type_service_impl::{createPJToDoTypeServiceImpl};
use view_models::to_do_type_view_model::PJToDoTypeViewModel;
use to_do_type::to_do_type::{ToDoTypeInsert, createToDoTypeInsert};
use common::{free_rust_object, free_rust_any_object};
use common::pj_logger::PJLogger;
use std::ffi::CString;
use std::sync::{Arc, Mutex};
use std::time::Duration;
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
    todo_typ_service_controller: *mut PJToDoTypeServiceController,
    pub todo_type_insert: *mut ToDoTypeInsert,
}

unsafe impl Send for PJToDoTypeController {}
unsafe impl Sync for PJToDoTypeController {}

impl PJToDoTypeController {
    fn new(delegate: IPJToDoTypeDelegate) -> PJToDoTypeController {
        let c_str = CString::new("".to_string()).unwrap();
        let controller = unsafe {
            PJToDoTypeController {
                delegate: delegate,
                todo_typ_service_controller: Box::into_raw(Box::new(
                    PJToDoTypeServiceController::new(),
                )),
                todo_type_insert: createToDoTypeInsert(c_str.into_raw()),
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
        // let i_delegate = Arc::new(Mutex::new(i_delegate));

        // let i_delegate = i_delegate.lock().unwrap();
        let result = insert_to_do_type(
            &(&(*self.todo_typ_service_controller)).todo_type_service,
            &(*to_do_type),
        );

        match result {
            Ok(_) => {
                (i_delegate.insert_result)(i_delegate.user, 0, true);
                pj_info!("insert to_do_type success");
            }
            Err(e) => {
                (i_delegate.insert_result)(i_delegate.user, -1, false);
                pj_error!("insert to_do_type fiald: {}", e);
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
#[allow(non_snake_case)]
pub unsafe extern "C" fn insertToDoType(
    ptr: *mut PJToDoTypeController,
    toDoType: *const ToDoTypeInsert,
) {
    if ptr.is_null() || toDoType.is_null() {
        pj_error!("ptr: *mut PJToDoTypeController is null!");
        return;
    }

    let controler = &mut *ptr;
    let toDoType = &*toDoType;

    thread::spawn(move || {
        println!("insertToDoType thread::spawn");
        thread::sleep(Duration::new(6, 0));
        controler.insert_to_do_type(toDoType);
    });
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn getToDoType(ptr: *const PJToDoTypeController) -> *const ToDoTypeInsert {
    assert!(!ptr.is_null());
    let controler = &*ptr;
    controler.todo_type_insert
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn free_rust_PJToDoTypeController(ptr: *mut PJToDoTypeController) {
    if ptr.is_null() {
        return;
    };
    Box::from_raw(ptr); //unsafe
}
