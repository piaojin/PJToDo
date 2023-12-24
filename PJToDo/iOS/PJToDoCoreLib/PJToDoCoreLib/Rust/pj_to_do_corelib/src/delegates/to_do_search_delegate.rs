use std::ops::Deref;
use libc::{c_void};
use std::marker::{Send, Sync};
use crate::to_do::to_do::{ToDoQuery};

#[repr(C)]
pub struct IPJToDoSearchDelegate {
    pub user: *mut c_void, //当前持有IPJToDoSearchDelegate对象的所有权者
    //释放内存回调，告诉当前持有IPJToDoSearchDelegate对象的所有权者做相应的处理
    pub destroy: extern "C" fn(user: *mut c_void),
    pub find_byTitle_result:
        extern "C" fn(user: *mut c_void, toDo: *mut ToDoQuery, isSuccess: bool),
    pub find_byLike_result: extern "C" fn(user: *mut c_void, isSuccess: bool),
}

impl Drop for IPJToDoSearchDelegate {
    fn drop(&mut self) {
        //IPJToDoSearchDelegate被释放，告诉当前持有IPJToDoSearchDelegate对象的所有权者做相应的处理
        println!("IPJToDoSearchDelegate -> drop");
    }
}

//该类的作用是当IPJToDoSearchDelegate被销毁时能够释放内存
pub struct IPJToDoSearchDelegateWrapper(pub *const IPJToDoSearchDelegate);

impl Deref for IPJToDoSearchDelegateWrapper {
    type Target = IPJToDoSearchDelegate;

    fn deref(&self) -> &IPJToDoSearchDelegate {
        unsafe { &(*self.0) }
    }
}

impl Drop for IPJToDoSearchDelegateWrapper {
    fn drop(&mut self) {
        //IPJToDoSearchDelegate被释放，告诉当前持有IPJToDoSearchDelegate对象的所有权者做相应的处理
        (self.destroy)(self.user);
        println!("IPJToDoSearchDelegateWrapper -> drop");
    }
}

unsafe impl Send for IPJToDoSearchDelegateWrapper {}
unsafe impl Sync for IPJToDoSearchDelegateWrapper {}
