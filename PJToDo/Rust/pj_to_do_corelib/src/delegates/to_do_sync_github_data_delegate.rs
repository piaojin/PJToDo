use std::ops::Deref;
use libc::{c_void};
use std::marker::{Send, Sync};
use to_do::to_do::{ToDoInsert};
use to_do_type::to_do_type::{ToDoTypeInsert};
use to_do_tag::to_do_tag::{ToDoTagInsert};
use mine::todo_settings::{ToDoSettingsInsert};

#[repr(C)]
pub struct IPJToDoSyncGitHubDataDelegate {
    pub user: *mut c_void, //当前持有IPJToDoSyncGitHubDataDelegate对象的所有权者
    //释放内存回调，告诉当前持有IPJToDoSyncGitHubDataDelegate对象的所有权者做相应的处理
    pub destroy: extern "C" fn(user: *mut c_void),
    pub sync_todo_data_result: extern "C" fn(user: *mut c_void, isSuccess: bool),
    pub sync_type_data_result: extern "C" fn(user: *mut c_void, isSuccess: bool),
    pub sync_tag_data_result: extern "C" fn(user: *mut c_void, isSuccess: bool),
    pub sync_settings_data_result: extern "C" fn(user: *mut c_void, isSuccess: bool),
}

impl Drop for IPJToDoSyncGitHubDataDelegate {
    fn drop(&mut self) {
        //IPJToDoSyncGitHubDataDelegate被释放，告诉当前持有IPJToDoSyncGitHubDataDelegate对象的所有权者做相应的处理
        println!("IPJToDoSyncGitHubDataDelegate -> drop");
    }
}

//该类的作用是当IPJToDoSyncGitHubDataDelegate被销毁时能够释放内存
pub struct IPJToDoSyncGitHubDataDelegateWrapper(pub *const IPJToDoSyncGitHubDataDelegate);

impl Deref for IPJToDoSyncGitHubDataDelegateWrapper {
    type Target = IPJToDoSyncGitHubDataDelegate;

    fn deref(&self) -> &IPJToDoSyncGitHubDataDelegate {
        unsafe { &(*self.0) }
    }
}

impl Drop for IPJToDoSyncGitHubDataDelegateWrapper {
    fn drop(&mut self) {
        //IPJToDoSyncGitHubDataDelegate被释放，告诉当前持有IPJToDoSyncGitHubDataDelegate对象的所有权者做相应的处理
        if self.user != std::ptr::null_mut() {
            (self.destroy)(self.user);
        }
        println!("IPJToDoSyncGitHubDataDelegateWrapper -> drop");
    }
}

unsafe impl Send for IPJToDoSyncGitHubDataDelegateWrapper {}
unsafe impl Sync for IPJToDoSyncGitHubDataDelegateWrapper {}
