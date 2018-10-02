/** ViewModel for to_do_type */
use to_do_type::to_do_type::ToDoTypeInsert;
use service::service_impl::to_do_type_service_impl::PJToDoTypeServiceImpl;
use service::to_do_type_service::PJToDoTypeService;
use libc::{c_void, c_char};
use std::ffi::CStr;

#[repr(C)]
#[derive(Debug)]
pub struct ToDoTypeServiceViewModel {
    insert_to_do_type: extern fn(to_do_type: ToDoTypeInsert),
}

impl ToDoTypeServiceViewModel {
    /**
     * 添加分类
     */
    pub fn insert_to_do_type(to_do_type: ToDoTypeInsert) {
        let result: Result<usize, String>  = PJToDoTypeServiceImpl::insert_to_do_type(to_do_type);
        //call back here
    }

    pub fn new() -> ToDoTypeServiceViewModel {
        ToDoTypeServiceViewModel{insert_to_do_type: insertToDoType}
    }
}

/*** extern "C" ***/

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn insertToDoType(toDoType: ToDoTypeInsert) {
    ToDoTypeServiceViewModel::insert_to_do_type(toDoType);
}

#[no_mangle]
pub extern "C" fn createToDoTypeServiceViewModel() -> *mut ToDoTypeServiceViewModel {
    Box::into_raw(Box::new(ToDoTypeServiceViewModel::new()))
}