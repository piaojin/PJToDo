use std::ops::Deref;
use libc::{c_void};
use std::marker::{Send, Sync};
use to_do_tag::to_do_tag::{ToDoTag};

#[repr(C)]
pub struct IPJToDoTagDelegate {
    pub user: *mut c_void, //当前持有IPJToDoTagDelegate对象的所有权者
    //释放内存回调，告诉当前持有IPJToDoTagDelegate对象的所有权者做相应的处理
    pub destroy: extern "C" fn(user: *mut c_void),
    pub insert_result: extern "C" fn(user: *mut c_void, isSuccess: bool),
    pub delete_result: extern "C" fn(user: *mut c_void, isSuccess: bool),
    pub update_result: extern "C" fn(user: *mut c_void, isSuccess: bool),
    pub find_byId_result: extern "C" fn(user: *mut c_void, toDoTag: *mut ToDoTag, isSuccess: bool),
    pub find_byName_result:
        extern "C" fn(user: *mut c_void, toDoTag: *mut ToDoTag, isSuccess: bool),
    pub fetch_data_result: extern "C" fn(user: *mut c_void, isSuccess: bool),
}

impl Drop for IPJToDoTagDelegate {
    fn drop(&mut self) {
        //IPJToDoTagDelegate被释放，告诉当前持有IPJToDoTagDelegate对象的所有权者做相应的处理
        println!("IPJToDoTagDelegate -> drop");
    }
}

//该类的作用是当IPJToDoTagDelegate被销毁时能够释放内存
pub struct IPJToDoTagDelegateWrapper(pub *const IPJToDoTagDelegate);

impl Deref for IPJToDoTagDelegateWrapper {
    type Target = IPJToDoTagDelegate;

    fn deref(&self) -> &IPJToDoTagDelegate {
        unsafe { &(*self.0) }
    }
}

impl Drop for IPJToDoTagDelegateWrapper {
    fn drop(&mut self) {
        //IPJToDoTagDelegate被释放，告诉当前持有IPJToDoTagDelegate对象的所有权者做相应的处理
        (self.destroy)(self.user);
        println!("IPJToDoTagDelegateWrapper -> drop");
    }
}

unsafe impl Send for IPJToDoTagDelegateWrapper {}
unsafe impl Sync for IPJToDoTagDelegateWrapper {}
