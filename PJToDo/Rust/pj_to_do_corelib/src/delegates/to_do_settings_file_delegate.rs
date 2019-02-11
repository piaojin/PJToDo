use std::ops::Deref;
use libc::{c_void};
use std::marker::{Send, Sync};

#[derive(Clone)]
#[repr(C)]
pub struct IPJToDoSettingsFileDelegate {
    pub user: *mut c_void, //当前持有IPJToDoSettingsFileDelegate对象的所有权者
    //释放内存回调，告诉当前持有IPJToDoSettingsFileDelegate对象的所有权者做相应的处理
    pub destroy: extern "C" fn(user: *mut c_void),
    pub action_result: extern "C" fn(user: *mut c_void, is_success: bool),
}

impl Drop for IPJToDoSettingsFileDelegate {
    fn drop(&mut self) {
        //IPJToDoSettingsFileDelegate被释放，告诉当前持有IPJToDoSettingsFileDelegate对象的所有权者做相应的处理
        println!("IPJToDoSettingsFileDelegate -> drop");
    }
}

//该类的作用是当IPJToDoSettingsFileDelegate被销毁时能够释放内存
#[derive(Clone)]
pub struct IPJToDoSettingsFileDelegateWrapper(pub IPJToDoSettingsFileDelegate);

impl Deref for IPJToDoSettingsFileDelegateWrapper {
    type Target = IPJToDoSettingsFileDelegate;

    fn deref(&self) -> &IPJToDoSettingsFileDelegate {
        &(self.0)
    }
}

impl Drop for IPJToDoSettingsFileDelegateWrapper {
    fn drop(&mut self) {
        //IPJToDoSettingsFileDelegate被释放，告诉当前持有IPJToDoSettingsFileDelegate对象的所有权者做相应的处理
        // (self.destroy)(self.user);
        println!("IPJToDoSettingsFileDelegateWrapper -> drop");
    }
}

unsafe impl Send for IPJToDoSettingsFileDelegateWrapper {}
unsafe impl Sync for IPJToDoSettingsFileDelegateWrapper {}
