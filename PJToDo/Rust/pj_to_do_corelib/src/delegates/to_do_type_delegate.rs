use std::ops::Deref;
use libc::{c_void, c_char};
use std::marker::{Send, Sync};

#[repr(C)]
pub struct IPJToDoTypeDelegate {
    pub user: *mut c_void, //当前持有IPJToDoTypeDelegate对象的所有权者
    //释放内存回调，告诉当前持有IPJToDoTypeDelegate对象的所有权者做相应的处理
    pub destroy: extern "C" fn(user: *mut c_void),
    pub insert_result: extern "C" fn(user: *mut c_void, _id: i32, isSuccess: bool),
}

impl Drop for IPJToDoTypeDelegate {
    fn drop(&mut self) {
        //IPJToDoTypeDelegate被释放，告诉当前持有IPJToDoTypeDelegate对象的所有权者做相应的处理
        println!("IPJToDoTypeDelegate -> drop");
    }
}

//该类的作用是当IPJToDoTypeDelegate被销毁时能够释放内存
pub struct IPJToDoTypeDelegateWrapper(pub *const IPJToDoTypeDelegate);

impl Deref for IPJToDoTypeDelegateWrapper {
    type Target = IPJToDoTypeDelegate;

    fn deref(&self) -> &IPJToDoTypeDelegate {
        unsafe { &(*self.0) }
    }
}

impl Drop for IPJToDoTypeDelegateWrapper {
    fn drop(&mut self) {
        //IPJToDoTypeDelegate被释放，告诉当前持有IPJToDoTypeDelegate对象的所有权者做相应的处理
        (self.destroy)(self.user);
        println!("IPJToDoTypeDelegateWrapper -> drop");
    }
}

unsafe impl Send for IPJToDoTypeDelegateWrapper {}
unsafe impl Sync for IPJToDoTypeDelegateWrapper {}
