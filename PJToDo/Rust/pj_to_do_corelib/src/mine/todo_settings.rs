extern crate serde_derive;
extern crate serde;
extern crate serde_json;

extern crate libc;
use self::libc::{c_char};
use std::ffi::{CStr, CString};

use db::tables::schema::{todosettings};
#[derive(
    Serialize, Deserialize, Debug, Default, PartialEq, Queryable, AsChangeset, Identifiable,
)]
#[table_name = "todosettings"]
pub struct ToDoSettings {
    pub id: i32,
    pub remind_email: String,
    pub remind_days: i32,
}

impl ToDoSettings {
    pub fn new(remind_email: String, remind_days: i32) -> ToDoSettings {
        ToDoSettings {
            id: -1,
            remind_email: remind_email,
            remind_days: remind_days,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Insertable)]
#[table_name = "todosettings"]
pub struct ToDoSettingsInsert {
    pub remind_email: String,
    pub remind_days: i32,
}

impl ToDoSettingsInsert {
    pub fn new(remind_email: String, remind_days: i32) -> ToDoSettingsInsert {
        ToDoSettingsInsert {
            remind_email: remind_email,
            remind_days: remind_days,
        }
    }
}

/*** extern "C" ***/

#[no_mangle]
pub unsafe extern "C" fn createToDoSettingsInsert(
    remind_email: *const c_char,
    remind_days: i32,
) -> *mut ToDoSettingsInsert {
    let remind_email = CStr::from_ptr(remind_email).to_string_lossy().into_owned(); //unsafe
    Box::into_raw(Box::new(ToDoSettingsInsert::new(remind_email, remind_days)))
}

#[no_mangle]
pub unsafe extern "C" fn createToDoSettings(
    remind_email: *const c_char,
    remind_days: i32,
) -> *mut ToDoSettings {
    let remind_email = CStr::from_ptr(remind_email).to_string_lossy().into_owned(); //unsafe
    Box::into_raw(Box::new(ToDoSettings::new(remind_email, remind_days)))
}

#[no_mangle]
pub unsafe extern "C" fn setToDoSettingsRemindEmail(
    ptr: *mut ToDoSettings,
    remind_email: *const c_char,
) {
    assert!(!ptr.is_null());
    let todo_settins = &mut *ptr;
    let remind_email = CStr::from_ptr(remind_email).to_string_lossy().into_owned();
    todo_settins.remind_email = remind_email;
}

#[no_mangle]
pub unsafe extern "C" fn getToDoSettingsRemindEmail(ptr: *const ToDoSettings) -> *const c_char {
    assert!(!ptr.is_null());
    let todo_settins = &*ptr;
    let remind_email = CString::new(todo_settins.remind_email.clone()).unwrap(); //unsafe
    remind_email.into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn setToDoSettingsId(ptr: *mut ToDoSettings, settins_id: i32) {
    assert!(!ptr.is_null());
    let todo_settins = &mut *ptr;
    todo_settins.id = settins_id;
}

#[no_mangle]
pub unsafe extern "C" fn getToDoSettingsId(ptr: *mut ToDoSettings) -> i32 {
    assert!(!ptr.is_null());
    let todo_settins = &mut *ptr;
    todo_settins.id
}

#[no_mangle]
pub unsafe extern "C" fn setToDoSettingsRemindDays(ptr: *mut ToDoSettings, remind_days: i32) {
    assert!(!ptr.is_null());
    let todo_settins = &mut *ptr;
    todo_settins.remind_days = remind_days;
}

#[no_mangle]
pub unsafe extern "C" fn getToDoSettingsRemindDays(ptr: *mut ToDoSettings) -> i32 {
    assert!(!ptr.is_null());
    let todo_settins = &mut *ptr;
    todo_settins.remind_days
}

#[no_mangle]
pub unsafe extern "C" fn setToDoSettingsInsertRemindEmail(
    ptr: *mut ToDoSettingsInsert,
    remind_email: *const c_char,
) {
    assert!(!ptr.is_null());
    let todo_settins = &mut *ptr;
    let remind_email = CStr::from_ptr(remind_email).to_string_lossy().into_owned();
    todo_settins.remind_email = remind_email;
}

#[no_mangle]
pub unsafe extern "C" fn getToDoSettingsInsertRemindEmail(
    ptr: *const ToDoSettingsInsert,
) -> *const c_char {
    assert!(!ptr.is_null());
    let todo_settins = &*ptr;
    let remind_email = CString::new(todo_settins.remind_email.clone()).unwrap(); //unsafe
    remind_email.into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn setToDoSettingsInsertRemindDays(
    ptr: *mut ToDoSettingsInsert,
    remind_days: i32,
) {
    assert!(!ptr.is_null());
    let todo_settins = &mut *ptr;
    todo_settins.remind_days = remind_days;
}

#[no_mangle]
pub unsafe extern "C" fn getToDoSettingsInsertRemindDays(ptr: *mut ToDoSettingsInsert) -> i32 {
    assert!(!ptr.is_null());
    let todo_settins = &mut *ptr;
    todo_settins.remind_days
}
