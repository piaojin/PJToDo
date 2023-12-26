use crate::{
    to_do::to_do::{ToDoQuery},
    common::utils::pj_utils::PJUtils,
};
use libc::{c_char};
use std::ffi::{CStr};

#[no_mangle]
pub unsafe extern "C" fn pj_set_todo_query_title(ptr: *mut ToDoQuery, title: *const c_char) {
    assert!(ptr != std::ptr::null_mut());
    let todo = &mut *ptr;
    let title = CStr::from_ptr(title).to_string_lossy().into_owned();
    todo.title = title;
}

#[no_mangle]
pub unsafe extern "C" fn pj_get_todo_query_title(ptr: *const ToDoQuery) -> *mut c_char {
    assert!(ptr != std::ptr::null_mut());
    let todo = &*ptr;
    let title = PJUtils::create_cstring_from(&todo.title);
    title.into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn pj_set_todo_query_id(ptr: *mut ToDoQuery, _id: i32) {
    assert!(ptr != std::ptr::null_mut());
    let todo = &mut *ptr;
    todo.id = _id;
}

#[no_mangle]
pub unsafe extern "C" fn pj_get_todo_query_id(ptr: *mut ToDoQuery) -> i32 {
    assert!(ptr != std::ptr::null_mut());
    let todo = &mut *ptr;
    todo.id
}

#[no_mangle]
pub unsafe extern "C" fn pj_set_todo_query_content(ptr: *mut ToDoQuery, content: *const c_char) {
    assert!(ptr != std::ptr::null_mut());
    let todo = &mut *ptr;
    let content = CStr::from_ptr(content).to_string_lossy().into_owned();
    todo.content = content;
}

#[no_mangle]
pub unsafe extern "C" fn pj_get_todo_query_content(ptr: *const ToDoQuery) -> *mut c_char {
    assert!(ptr != std::ptr::null_mut());
    let todo = &*ptr;
    let content = PJUtils::create_cstring_from(&todo.content);
    content.into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn pj_set_todo_query_duetime(ptr: *mut ToDoQuery, due_time: *const c_char) {
    assert!(ptr != std::ptr::null_mut());
    let todo = &mut *ptr;
    let due_time = CStr::from_ptr(due_time).to_string_lossy().into_owned();
    todo.due_time = due_time;
}

#[no_mangle]
pub unsafe extern "C" fn pj_get_todo_query_duetime(ptr: *const ToDoQuery) -> *mut c_char {
    assert!(ptr != std::ptr::null_mut());
    let todo = &*ptr;
    let due_time = PJUtils::create_cstring_from(&todo.due_time);
    due_time.into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn pj_set_todo_query_remind_time(
    ptr: *mut ToDoQuery,
    remind_time: *const c_char,
) {
    assert!(ptr != std::ptr::null_mut());
    let todo = &mut *ptr;
    let remind_time = CStr::from_ptr(remind_time).to_string_lossy().into_owned();
    todo.remind_time = remind_time;
}

#[no_mangle]
pub unsafe extern "C" fn pj_get_todo_query_remind_time(ptr: *const ToDoQuery) -> *mut c_char {
    assert!(ptr != std::ptr::null_mut());
    let todo = &*ptr;
    let remind_time = PJUtils::create_cstring_from(&todo.remind_time);
    remind_time.into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn pj_set_todo_query_create_time(
    ptr: *mut ToDoQuery,
    create_time: *const c_char,
) {
    assert!(ptr != std::ptr::null_mut());
    let todo = &mut *ptr;
    let create_time = CStr::from_ptr(create_time).to_string_lossy().into_owned();
    todo.create_time = create_time;
}

#[no_mangle]
pub unsafe extern "C" fn pj_get_todo_query_create_time(ptr: *const ToDoQuery) -> *mut c_char {
    assert!(ptr != std::ptr::null_mut());
    let todo = &*ptr;
    let create_time = PJUtils::create_cstring_from(&todo.create_time); //unsafe
    create_time.into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn pj_set_todo_query_update_time(
    ptr: *mut ToDoQuery,
    update_time: *const c_char,
) {
    assert!(ptr != std::ptr::null_mut());
    let todo = &mut *ptr;
    let update_time = CStr::from_ptr(update_time).to_string_lossy().into_owned();
    todo.update_time = update_time;
}

#[no_mangle]
pub unsafe extern "C" fn pj_get_todo_query_update_time(ptr: *const ToDoQuery) -> *mut c_char {
    assert!(ptr != std::ptr::null_mut());
    let todo = &*ptr;
    let update_time = PJUtils::create_cstring_from(&todo.update_time); //unsafe
    update_time.into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn pj_set_todo_query_todo_type_id(ptr: *mut ToDoQuery, to_do_type_id: i32) {
    assert!(ptr != std::ptr::null_mut());
    let todo = &mut *ptr;
    todo.to_do_type_id = to_do_type_id;
}

#[no_mangle]
pub unsafe extern "C" fn pj_get_todo_query_todo_type_id(ptr: *mut ToDoQuery) -> i32 {
    assert!(ptr != std::ptr::null_mut());
    let todo = &mut *ptr;
    todo.to_do_type_id
}

#[no_mangle]
pub unsafe extern "C" fn pj_set_todo_query_todo_tag_id(ptr: *mut ToDoQuery, to_do_tag_id: i32) {
    assert!(ptr != std::ptr::null_mut());
    let todo = &mut *ptr;
    todo.to_do_tag_id = to_do_tag_id;
}

#[no_mangle]
pub unsafe extern "C" fn pj_get_todo_query_todo_priority(ptr: *mut ToDoQuery) -> i32 {
    assert!(ptr != std::ptr::null_mut());
    let todo = &mut *ptr;
    todo.priority
}

#[no_mangle]
pub unsafe extern "C" fn pj_set_todo_query_todo_priority(ptr: *mut ToDoQuery, priority: i32) {
    assert!(ptr != std::ptr::null_mut());
    let todo = &mut *ptr;
    todo.priority = priority;
}

#[no_mangle]
pub unsafe extern "C" fn pj_get_todo_query_todo_tag_id(ptr: *mut ToDoQuery) -> i32 {
    assert!(ptr != std::ptr::null_mut());
    let todo = &mut *ptr;
    todo.to_do_tag_id
}

#[no_mangle]
pub unsafe extern "C" fn pj_set_todo_query_state(ptr: *mut ToDoQuery, state: i32) {
    assert!(ptr != std::ptr::null_mut());
    let todo = &mut *ptr;
    todo.state = state;
}

#[no_mangle]
pub unsafe extern "C" fn pj_get_todo_query_state(ptr: *mut ToDoQuery) -> i32 {
    assert!(ptr != std::ptr::null_mut());
    let todo = &mut *ptr;
    todo.state
}
