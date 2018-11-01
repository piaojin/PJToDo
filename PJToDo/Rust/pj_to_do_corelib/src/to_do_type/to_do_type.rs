extern crate serde_derive;
extern crate serde;
extern crate serde_json;

extern crate libc;
use self::libc::{c_char};
use std::ffi::{CStr, CString};

use db::tables::schema::{todotype};
use std::marker::{Send, Sync};
// #[primary_key(id)]
// #[column_name(barId)]
#[derive(
    Serialize, Deserialize, Debug, Default, PartialEq, Queryable, AsChangeset, Identifiable,
)]
#[table_name = "todotype"]
pub struct ToDoType {
    pub id: i32,
    pub type_name: String,
}

impl ToDoType {
    pub fn new(type_name: String) -> ToDoType {
        ToDoType {
            id: -1,
            type_name: type_name,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Insertable)]
#[table_name = "todotype"]
pub struct ToDoTypeInsert {
    pub type_name: String,
}

impl ToDoTypeInsert {
    pub fn new(type_name: String) -> ToDoTypeInsert {
        ToDoTypeInsert {
            type_name: type_name,
        }
    }
}

// impl Drop for ToDoTypeInsert {
//     fn drop(&mut self) {
//         println!("ToDoTypeInsert -> drop");
//     }
// }

/*** extern "C" ***/

#[no_mangle]
pub unsafe extern "C" fn createToDoTypeInsert(type_name: *const c_char) -> *mut ToDoTypeInsert {
    let type_name = CStr::from_ptr(type_name).to_string_lossy().into_owned(); //unsafe
    Box::into_raw(Box::new(ToDoTypeInsert::new(type_name)))
}

#[no_mangle]
pub unsafe extern "C" fn createToDoType(type_name: *const c_char) -> *mut ToDoType {
    let type_name = CStr::from_ptr(type_name).to_string_lossy().into_owned(); //unsafe
    Box::into_raw(Box::new(ToDoType::new(type_name)))
}

#[no_mangle]
pub unsafe extern "C" fn setToDoTypeTypeName(ptr: *mut ToDoType, type_name: *const c_char) {
    assert!(!ptr.is_null());
    let todo_type = &mut *ptr;
    let type_name = CStr::from_ptr(type_name).to_string_lossy().into_owned();
    todo_type.type_name = type_name;
}

#[no_mangle]
pub unsafe extern "C" fn getToDoTypeTypeName(ptr: *const ToDoType) -> *const c_char {
    assert!(!ptr.is_null());
    let todo_type = &*ptr;
    let type_name = CString::new(todo_type.type_name.clone()).unwrap(); //unsafe
    type_name.into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn setToDoTypeTypeId(ptr: *mut ToDoType, type_id: i32) {
    assert!(!ptr.is_null());
    let todo_type = &mut *ptr;
    todo_type.id = type_id;
}

#[no_mangle]
pub unsafe extern "C" fn getToDoTypeTypeId(ptr: *mut ToDoType) -> i32 {
    assert!(!ptr.is_null());
    let todo_type = &mut *ptr;
    todo_type.id
}

#[no_mangle]
pub unsafe extern "C" fn free_rust_ToDoType(ptr: *mut ToDoType) {
    if ptr.is_null() {
        return;
    };
    Box::from_raw(ptr); //unsafe
}

#[no_mangle]
pub unsafe extern "C" fn free_rust_ToDoTypeInsert(ptr: *mut ToDoTypeInsert) {
    if ptr.is_null() {
        return;
    };
    Box::from_raw(ptr); //unsafe
}
