use std::ops::Deref;
use libc::{c_void};
use std::marker::{Send, Sync};

#[repr(C)]
pub struct IPJToDoSettingsDelegate {
    pub user: *mut c_void, //当前持有IPJToDoSettingsDelegate对象的所有权者
    //释放内存回调，告诉当前持有IPJToDoSettingsDelegate对象的所有权者做相应的处理
    pub destroy: extern "C" fn(user: *mut c_void),
    pub insert_result: extern "C" fn(user: *mut c_void, isSuccess: bool),
    pub delete_result: extern "C" fn(user: *mut c_void, isSuccess: bool),
    pub update_result: extern "C" fn(user: *mut c_void, isSuccess: bool),
    pub fetch_data_result: extern "C" fn(user: *mut c_void, isSuccess: bool),
}

impl Drop for IPJToDoSettingsDelegate {
    fn drop(&mut self) {
        //IPJToDoSettingsDelegate被释放，告诉当前持有IPJToDoSettingsDelegate对象的所有权者做相应的处理
        println!("IPJToDoSettingsDelegate -> drop");
    }
}

//该类的作用是当IPJToDoSettingsDelegate被销毁时能够释放内存
pub struct IPJToDoSettingsDelegateWrapper(pub *const IPJToDoSettingsDelegate);

impl Deref for IPJToDoSettingsDelegateWrapper {
    type Target = IPJToDoSettingsDelegate;

    fn deref(&self) -> &IPJToDoSettingsDelegate {
        unsafe { &(*self.0) }
    }
}

impl Drop for IPJToDoSettingsDelegateWrapper {
    fn drop(&mut self) {
        //IPJToDoSettingsDelegate被释放，告诉当前持有IPJToDoSettingsDelegate对象的所有权者做相应的处理
        if self.user != std::ptr::null_mut() {
            (self.destroy)(self.user);
        }
        println!("IPJToDoSettingsDelegateWrapper -> drop");
    }
}

unsafe impl Send for IPJToDoSettingsDelegateWrapper {}
unsafe impl Sync for IPJToDoSettingsDelegateWrapper {}
