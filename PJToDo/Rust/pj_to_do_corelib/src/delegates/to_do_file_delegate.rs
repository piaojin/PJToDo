use std::ops::Deref;
use libc::{c_void};
use std::marker::{Send, Sync};

#[derive(Clone)]
#[repr(C)]
pub struct IPJToDoFileDelegate {
    pub user: *mut c_void, //当前持有IPJToDoFileDelegate对象的所有权者
    //释放内存回调，告诉当前持有IPJToDoFileDelegate对象的所有权者做相应的处理
    pub destroy: extern "C" fn(user: *mut c_void),
    pub action_result: extern "C" fn(user: *mut c_void, is_success: bool),
}

impl Drop for IPJToDoFileDelegate {
    fn drop(&mut self) {
        //IPJToDoFileDelegate被释放，告诉当前持有IPJToDoFileDelegate对象的所有权者做相应的处理
        println!("IPJToDoFileDelegate -> drop");
    }
}

//该类的作用是当IPJToDoFileDelegate被销毁时能够释放内存
#[derive(Clone)]
pub struct IPJToDoFileDelegateWrapper(pub IPJToDoFileDelegate);

impl Deref for IPJToDoFileDelegateWrapper {
    type Target = IPJToDoFileDelegate;

    fn deref(&self) -> &IPJToDoFileDelegate {
        &(self.0)
    }
}

impl Drop for IPJToDoFileDelegateWrapper {
    fn drop(&mut self) {
        //IPJToDoFileDelegate被释放，告诉当前持有IPJToDoFileDelegate对象的所有权者做相应的处理
        // (self.destroy)(self.user);
        println!("IPJToDoFileDelegateWrapper -> drop");
    }
}

unsafe impl Send for IPJToDoFileDelegateWrapper {}
unsafe impl Sync for IPJToDoFileDelegateWrapper {}
