extern crate serde_derive;
extern crate serde;
extern crate serde_json;

extern crate libc;
use self::libc::c_char;
use std::ffi::{CStr};

use crate::{db::tables::schema::todosettings, common::utils::pj_utils::PJUtils};
#[derive(
    Serialize, Deserialize, Debug, Default, PartialEq, Queryable, AsChangeset, Identifiable,
)]
#[diesel(table_name = todosettings)]
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
#[diesel(table_name = todosettings)]
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
pub unsafe extern "C" fn pj_create_ToDoSettingsInsert(
    remind_email: *const c_char,
    remind_days: i32,
) -> *mut ToDoSettingsInsert {
    let remind_email = CStr::from_ptr(remind_email).to_string_lossy().into_owned(); //unsafe
    Box::into_raw(Box::new(ToDoSettingsInsert::new(remind_email, remind_days)))
}

#[no_mangle]
pub unsafe extern "C" fn pj_create_ToDoSettings(
    remind_email: *const c_char,
    remind_days: i32,
) -> *mut ToDoSettings {
    let remind_email = CStr::from_ptr(remind_email).to_string_lossy().into_owned(); //unsafe
    Box::into_raw(Box::new(ToDoSettings::new(remind_email, remind_days)))
}

#[no_mangle]
pub unsafe extern "C" fn pj_set_todo_settings_remind_email(
    ptr: *mut ToDoSettings,
    remind_email: *const c_char,
) {
    assert!(ptr != std::ptr::null_mut());
    let todo_settins = &mut *ptr;
    let remind_email = CStr::from_ptr(remind_email).to_string_lossy().into_owned();
    todo_settins.remind_email = remind_email;
}

#[no_mangle]
pub unsafe extern "C" fn pj_get_todo_settings_remind_email(
    ptr: *const ToDoSettings,
) -> *mut c_char {
    assert!(ptr != std::ptr::null_mut());
    let todo_settings = &*ptr;
    let remind_email = PJUtils::create_cstring_from(&todo_settings.remind_email); //unsafe
    remind_email.into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn pj_set_todo_settings_id(ptr: *mut ToDoSettings, settins_id: i32) {
    assert!(ptr != std::ptr::null_mut());
    let todo_settins = &mut *ptr;
    todo_settins.id = settins_id;
}

#[no_mangle]
pub unsafe extern "C" fn pj_get_todo_settings_id(ptr: *mut ToDoSettings) -> i32 {
    assert!(ptr != std::ptr::null_mut());
    let todo_settins = &mut *ptr;
    todo_settins.id
}

#[no_mangle]
pub unsafe extern "C" fn pj_set_todo_settings_remind_days(
    ptr: *mut ToDoSettings,
    remind_days: i32,
) {
    assert!(ptr != std::ptr::null_mut());
    let todo_settins = &mut *ptr;
    todo_settins.remind_days = remind_days;
}

#[no_mangle]
pub unsafe extern "C" fn pj_get_todo_settings_remind_days(ptr: *mut ToDoSettings) -> i32 {
    assert!(ptr != std::ptr::null_mut());
    let todo_settins = &mut *ptr;
    todo_settins.remind_days
}

#[no_mangle]
pub unsafe extern "C" fn pj_set_todo_settings_insert_remind_email(
    ptr: *mut ToDoSettingsInsert,
    remind_email: *const c_char,
) {
    assert!(ptr != std::ptr::null_mut());
    let todo_settins = &mut *ptr;
    let remind_email = CStr::from_ptr(remind_email).to_string_lossy().into_owned();
    todo_settins.remind_email = remind_email;
}

#[no_mangle]
pub unsafe extern "C" fn pj_get_todo_settings_insert_remind_email(
    ptr: *const ToDoSettingsInsert,
) -> *mut c_char {
    assert!(ptr != std::ptr::null_mut());
    let todo_settings = &*ptr;
    let remind_email = PJUtils::create_cstring_from(&todo_settings.remind_email);
    remind_email.into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn pj_set_todo_settings_insert_remind_days(
    ptr: *mut ToDoSettingsInsert,
    remind_days: i32,
) {
    assert!(ptr != std::ptr::null_mut());
    let todo_settins = &mut *ptr;
    todo_settins.remind_days = remind_days;
}

#[no_mangle]
pub unsafe extern "C" fn pj_get_todo_settings_insert_remind_days(
    ptr: *mut ToDoSettingsInsert,
) -> i32 {
    assert!(ptr != std::ptr::null_mut());
    let todo_settins = &mut *ptr;
    todo_settins.remind_days
}
