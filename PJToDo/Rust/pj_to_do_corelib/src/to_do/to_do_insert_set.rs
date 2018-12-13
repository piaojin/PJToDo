use to_do::to_do::{ToDoInsert};
use libc::{c_char};
use std::ffi::{CStr, CString};

#[no_mangle]
pub unsafe extern "C" fn setToDoInsertTitle(ptr: *mut ToDoInsert, title: *const c_char) {
    assert!(ptr != std::ptr::null_mut());
    let todo = &mut *ptr;
    let title = CStr::from_ptr(title).to_string_lossy().into_owned();
    todo.title = title;
}

#[no_mangle]
pub unsafe extern "C" fn getToDoInsertTitle(ptr: *const ToDoInsert) -> *mut c_char {
    assert!(ptr != std::ptr::null_mut());
    let todo = &*ptr;
    let title = CString::new(todo.title.clone()).unwrap(); //unsafe
    title.into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn setToDoInsertContent(ptr: *mut ToDoInsert, content: *const c_char) {
    assert!(ptr != std::ptr::null_mut());
    let todo = &mut *ptr;
    let content = CStr::from_ptr(content).to_string_lossy().into_owned();
    todo.content = content;
}

#[no_mangle]
pub unsafe extern "C" fn getToDoInsertContent(ptr: *const ToDoInsert) -> *mut c_char {
    assert!(ptr != std::ptr::null_mut());
    let todo = &*ptr;
    let content = CString::new(todo.content.clone()).unwrap(); //unsafe
    content.into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn setToDoInsertDueTime(ptr: *mut ToDoInsert, due_time: *const c_char) {
    assert!(ptr != std::ptr::null_mut());
    let todo = &mut *ptr;
    let due_time = CStr::from_ptr(due_time).to_string_lossy().into_owned();
    todo.due_time = due_time;
}

#[no_mangle]
pub unsafe extern "C" fn getToDoInsertDueTime(ptr: *const ToDoInsert) -> *mut c_char {
    assert!(ptr != std::ptr::null_mut());
    let todo = &*ptr;
    let due_time = CString::new(todo.due_time.clone()).unwrap(); //unsafe
    due_time.into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn setToDoInsertRemindTime(ptr: *mut ToDoInsert, remind_time: *const c_char) {
    assert!(ptr != std::ptr::null_mut());
    let todo = &mut *ptr;
    let remind_time = CStr::from_ptr(remind_time).to_string_lossy().into_owned();
    todo.remind_time = remind_time;
}

#[no_mangle]
pub unsafe extern "C" fn getToDoInsertRemindTime(ptr: *const ToDoInsert) -> *mut c_char {
    assert!(ptr != std::ptr::null_mut());
    let todo = &*ptr;
    let remind_time = CString::new(todo.remind_time.clone()).unwrap(); //unsafe
    remind_time.into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn setToDoInsertCreateTime(ptr: *mut ToDoInsert, create_time: *const c_char) {
    assert!(ptr != std::ptr::null_mut());
    let todo = &mut *ptr;
    let create_time = CStr::from_ptr(create_time).to_string_lossy().into_owned();
    todo.create_time = create_time;
}

#[no_mangle]
pub unsafe extern "C" fn getToDoInsertCreateTime(ptr: *const ToDoInsert) -> *mut c_char {
    assert!(ptr != std::ptr::null_mut());
    let todo = &*ptr;
    let create_time = CString::new(todo.create_time.clone()).unwrap(); //unsafe
    create_time.into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn setToDoInsertUpdateTime(ptr: *mut ToDoInsert, update_time: *const c_char) {
    assert!(ptr != std::ptr::null_mut());
    let todo = &mut *ptr;
    let update_time = CStr::from_ptr(update_time).to_string_lossy().into_owned();
    todo.update_time = update_time;
}

#[no_mangle]
pub unsafe extern "C" fn getToDoInsertUpdateTime(ptr: *const ToDoInsert) -> *mut c_char {
    assert!(ptr != std::ptr::null_mut());
    let todo = &*ptr;
    let update_time = CString::new(todo.update_time.clone()).unwrap(); //unsafe
    update_time.into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn setToDoInsert_ToDoTypeId(ptr: *mut ToDoInsert, to_do_type_id: i32) {
    assert!(ptr != std::ptr::null_mut());
    let todo = &mut *ptr;
    todo.to_do_type_id = to_do_type_id;
}

#[no_mangle]
pub unsafe extern "C" fn getToDoInsert_ToDoTypeId(ptr: *mut ToDoInsert) -> i32 {
    assert!(ptr != std::ptr::null_mut());
    let todo = &mut *ptr;
    todo.to_do_type_id
}

#[no_mangle]
pub unsafe extern "C" fn setToDoInsert_ToDoTagId(ptr: *mut ToDoInsert, to_do_tag_id: i32) {
    assert!(ptr != std::ptr::null_mut());
    let todo = &mut *ptr;
    todo.to_do_tag_id = to_do_tag_id;
}

#[no_mangle]
pub unsafe extern "C" fn getToDoInsert_ToDoTagId(ptr: *mut ToDoInsert) -> i32 {
    assert!(ptr != std::ptr::null_mut());
    let todo = &mut *ptr;
    todo.to_do_tag_id
}

#[no_mangle]
pub unsafe extern "C" fn setToDoInsertState(ptr: *mut ToDoInsert, state: i32) {
    assert!(ptr != std::ptr::null_mut());
    let todo = &mut *ptr;
    todo.state = state;
}

#[no_mangle]
pub unsafe extern "C" fn getToDoInsertState(ptr: *mut ToDoInsert) -> i32 {
    assert!(ptr != std::ptr::null_mut());
    let todo = &mut *ptr;
    todo.state
}
