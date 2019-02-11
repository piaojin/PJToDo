use std::ops::Deref;
use libc::{c_void};
use std::marker::{Send, Sync};

#[derive(Clone)]
#[repr(C)]
pub struct IPJToDoTypeFileDelegate {
    pub user: *mut c_void, //当前持有IPJToDoTypeFileDelegate对象的所有权者
    //释放内存回调，告诉当前持有IPJToDoTypeFileDelegate对象的所有权者做相应的处理
    pub destroy: extern "C" fn(user: *mut c_void),
    pub action_result: extern "C" fn(user: *mut c_void, is_success: bool),
}

impl Drop for IPJToDoTypeFileDelegate {
    fn drop(&mut self) {
        //IPJToDoTypeFileDelegate被释放，告诉当前持有IPJToDoTypeFileDelegate对象的所有权者做相应的处理
        println!("IPJToDoTypeFileDelegate -> drop");
    }
}

//该类的作用是当IPJToDoTypeFileDelegate被销毁时能够释放内存
#[derive(Clone)]
pub struct IPJToDoTypeFileDelegateWrapper(pub IPJToDoTypeFileDelegate);

impl Deref for IPJToDoTypeFileDelegateWrapper {
    type Target = IPJToDoTypeFileDelegate;

    fn deref(&self) -> &IPJToDoTypeFileDelegate {
        &(self.0)
    }
}

impl Drop for IPJToDoTypeFileDelegateWrapper {
    fn drop(&mut self) {
        //IPJToDoTypeFileDelegate被释放，告诉当前持有IPJToDoTypeFileDelegate对象的所有权者做相应的处理
        // (self.destroy)(self.user);
        println!("IPJToDoTypeFileDelegateWrapper -> drop");
    }
}

unsafe impl Send for IPJToDoTypeFileDelegateWrapper {}
unsafe impl Sync for IPJToDoTypeFileDelegateWrapper {}
