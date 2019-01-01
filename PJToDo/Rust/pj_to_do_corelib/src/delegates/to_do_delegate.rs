use std::ops::Deref;
use libc::{c_void};
use std::marker::{Send, Sync};
use to_do::to_do::{ToDoQuery};

#[repr(C)]
pub struct IPJToDoDelegate {
    pub user: *mut c_void, //当前持有IPJToDoDelegate对象的所有权者
    //释放内存回调，告诉当前持有IPJToDoDelegate对象的所有权者做相应的处理
    pub destroy: extern "C" fn(user: *mut c_void),
    pub insert_result: extern "C" fn(user: *mut c_void, isSuccess: bool),
    pub delete_result: extern "C" fn(user: *mut c_void, isSuccess: bool),
    pub update_result: extern "C" fn(user: *mut c_void, isSuccess: bool),
    pub find_byId_result: extern "C" fn(user: *mut c_void, toDo: *mut ToDoQuery, isSuccess: bool),
    pub fetch_data_result: extern "C" fn(user: *mut c_void, isSuccess: bool),
    pub update_overdue_todos: extern "C" fn(user: *mut c_void, isSuccess: bool),
}

impl Drop for IPJToDoDelegate {
    fn drop(&mut self) {
        //IPJToDoDelegate被释放，告诉当前持有IPJToDoDelegate对象的所有权者做相应的处理
        println!("IPJToDoDelegate -> drop");
    }
}

//该类的作用是当IPJToDoDelegate被销毁时能够释放内存
pub struct IPJToDoDelegateWrapper(pub *const IPJToDoDelegate);

impl Deref for IPJToDoDelegateWrapper {
    type Target = IPJToDoDelegate;

    fn deref(&self) -> &IPJToDoDelegate {
        unsafe { &(*self.0) }
    }
}

impl Drop for IPJToDoDelegateWrapper {
    fn drop(&mut self) {
        //IPJToDoDelegate被释放，告诉当前持有IPJToDoDelegate对象的所有权者做相应的处理
        (self.destroy)(self.user);
        println!("IPJToDoDelegateWrapper -> drop");
    }
}

unsafe impl Send for IPJToDoDelegateWrapper {}
unsafe impl Sync for IPJToDoDelegateWrapper {}
