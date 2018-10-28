/** ViewModel for to_do_type */
use to_do_type::to_do_type::ToDoTypeInsert;
use service::service_impl::to_do_type_service_impl::PJToDoTypeServiceImpl;
use service::to_do_type_service::PJToDoTypeService;
use libc::{c_void, c_char};
use std::ffi::CStr;

#[repr(C)]
#[derive(Debug)]
pub struct PJToDoTypeViewModel {
    insert_to_do_type: extern "C" fn(to_do_type: ToDoTypeInsert),
}

impl PJToDoTypeViewModel {
    /**
     * 添加分类
     */
    pub fn insert_to_do_type(to_do_type: ToDoTypeInsert) {
        // let result: Result<usize, String> = PJToDoTypeServiceImpl::insert_to_do_type(to_do_type);
        //call back here
    }

    pub fn new() -> PJToDoTypeViewModel {
        PJToDoTypeViewModel {
            insert_to_do_type: insertToDoType2,
        }
    }
}

/*** extern "C" ***/

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn insertToDoType2(toDoType: ToDoTypeInsert) {
    PJToDoTypeViewModel::insert_to_do_type(toDoType);
}

#[no_mangle]
pub extern "C" fn createPJToDoTypeViewModel() -> *mut PJToDoTypeViewModel {
    Box::into_raw(Box::new(PJToDoTypeViewModel::new()))
}
