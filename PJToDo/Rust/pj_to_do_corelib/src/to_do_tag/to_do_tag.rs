extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate libc;

use self::libc::{c_char};
use std::ffi::{CStr, CString};
use db::tables::schema::{todotag};

#[derive(
    Serialize, Deserialize, Debug, Default, PartialEq, Queryable, AsChangeset, Identifiable,
)]
#[table_name = "todotag"]
pub struct ToDoTag {
    pub id: i32,
    pub tag_name: String,
}

impl ToDoTag {
    pub fn new(tag_name: String) -> ToDoTag {
        ToDoTag {
            id: -1,
            tag_name: tag_name,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Insertable)]
#[table_name = "todotag"]
pub struct ToDoTagInsert {
    pub tag_name: String,
}

impl ToDoTagInsert {
    pub fn new(tag_name: String) -> ToDoTagInsert {
        ToDoTagInsert { tag_name: tag_name }
    }
}

/*** extern "C" ***/

#[no_mangle]
pub unsafe extern "C" fn createToDoTagInsert(tag_name: *const c_char) -> *mut ToDoTagInsert {
    let tag_name = CStr::from_ptr(tag_name).to_string_lossy().into_owned(); //unsafe
    Box::into_raw(Box::new(ToDoTagInsert::new(tag_name)))
}

#[no_mangle]
pub unsafe extern "C" fn createToDoTag(tag_name: *const c_char) -> *mut ToDoTag {
    let tag_name = CStr::from_ptr(tag_name).to_string_lossy().into_owned(); //unsafe
    Box::into_raw(Box::new(ToDoTag::new(tag_name)))
}

#[no_mangle]
pub unsafe extern "C" fn setToDoTagName(ptr: *mut ToDoTag, tag_name: *const c_char) {
    assert!(!ptr.is_null());
    let todo_tag = &mut *ptr;
    let tag_name = CStr::from_ptr(tag_name).to_string_lossy().into_owned();
    todo_tag.tag_name = tag_name;
}

#[no_mangle]
pub unsafe extern "C" fn getToDoTagName(ptr: *const ToDoTag) -> *const c_char {
    assert!(!ptr.is_null());
    let todo_tag = &*ptr;
    let tag_name = CString::new(todo_tag.tag_name.clone()).unwrap(); //unsafe
    tag_name.into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn setToDoTagId(ptr: *mut ToDoTag, type_id: i32) {
    assert!(!ptr.is_null());
    let todo_tag = &mut *ptr;
    todo_tag.id = type_id;
}

#[no_mangle]
pub unsafe extern "C" fn getToDoTagId(ptr: *mut ToDoTag) -> i32 {
    assert!(!ptr.is_null());
    let todo_tag = &mut *ptr;
    todo_tag.id
}

#[no_mangle]
pub unsafe extern "C" fn setToDoTagInsertName(ptr: *mut ToDoTagInsert, tag_name: *const c_char) {
    assert!(!ptr.is_null());
    let todo_tag = &mut *ptr;
    let tag_name = CStr::from_ptr(tag_name).to_string_lossy().into_owned();
    todo_tag.tag_name = tag_name;
}

#[no_mangle]
pub unsafe extern "C" fn getToDoTagInsertName(ptr: *const ToDoTagInsert) -> *const c_char {
    assert!(!ptr.is_null());
    let todo_tag = &*ptr;
    let tag_name = CString::new(todo_tag.tag_name.clone()).unwrap(); //unsafe
    tag_name.into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn free_rust_ToDoTag(ptr: *mut ToDoTag) {
    if ptr.is_null() {
        return;
    };
    Box::from_raw(ptr); //unsafe
}

#[no_mangle]
pub unsafe extern "C" fn free_rust_ToDoTagInsert(ptr: *mut ToDoTagInsert) {
    if ptr.is_null() {
        return;
    };
    Box::from_raw(ptr); //unsafe
}
