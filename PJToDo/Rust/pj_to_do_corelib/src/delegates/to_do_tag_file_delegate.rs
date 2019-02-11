use std::ops::Deref;
use libc::{c_void};
use std::marker::{Send, Sync};

#[derive(Clone)]
#[repr(C)]
pub struct IPJToDoTagFileDelegate {
    pub user: *mut c_void, //当前持有IPJToDoTagFileDelegate对象的所有权者
    //释放内存回调，告诉当前持有IPJToDoTagFileDelegate对象的所有权者做相应的处理
    pub destroy: extern "C" fn(user: *mut c_void),
    pub action_result: extern "C" fn(user: *mut c_void, is_success: bool),
}

impl Drop for IPJToDoTagFileDelegate {
    fn drop(&mut self) {
        //IPJToDoTagFileDelegate被释放，告诉当前持有IPJToDoTagFileDelegate对象的所有权者做相应的处理
        println!("IPJToDoTagFileDelegate -> drop");
    }
}

//该类的作用是当IPJToDoTagFileDelegate被销毁时能够释放内存
#[derive(Clone)]
pub struct IPJToDoTagFileDelegateWrapper(pub IPJToDoTagFileDelegate);

impl Deref for IPJToDoTagFileDelegateWrapper {
    type Target = IPJToDoTagFileDelegate;

    fn deref(&self) -> &IPJToDoTagFileDelegate {
        &(self.0)
    }
}

impl Drop for IPJToDoTagFileDelegateWrapper {
    fn drop(&mut self) {
        //IPJToDoTagFileDelegate被释放，告诉当前持有IPJToDoTagFileDelegate对象的所有权者做相应的处理
        // (self.destroy)(self.user);
        println!("IPJToDoTagFileDelegateWrapper -> drop");
    }
}

unsafe impl Send for IPJToDoTagFileDelegateWrapper {}
unsafe impl Sync for IPJToDoTagFileDelegateWrapper {}
