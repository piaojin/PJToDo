use std::ops::Deref;
use libc::{c_void};
use std::marker::{Send, Sync};

#[repr(C)]
#[derive(Clone)]
pub struct IPJToDoHttpRequestDelegate {
    pub user: *mut c_void, //当前持有IPJToDoHttpRequestDelegate对象的所有权者
    //释放内存回调，告诉当前持有IPJToDoHttpRequestDelegate对象的所有权者做相应的处理
    pub destroy: extern "C" fn(user: *mut c_void),
    pub request_result: extern "C" fn(user: *mut c_void, data: *mut c_void, isSuccess: bool),
}

impl Drop for IPJToDoHttpRequestDelegate {
    fn drop(&mut self) {
        //IPJToDoHttpRequestDelegate被释放，告诉当前持有IPJToDoHttpRequestDelegate对象的所有权者做相应的处理
        println!("IPJToDoHttpRequestDelegate -> drop");
    }
}

//该类的作用是当IPJToDoHttpRequestDelegate被销毁时能够释放内存
#[derive(Clone)]
pub struct IPJToDoHttpRequestDelegateWrapper(pub IPJToDoHttpRequestDelegate);

impl Deref for IPJToDoHttpRequestDelegateWrapper {
    type Target = IPJToDoHttpRequestDelegate;

    fn deref(&self) -> &IPJToDoHttpRequestDelegate {
        &(self.0)
    }
}

impl Drop for IPJToDoHttpRequestDelegateWrapper {
    fn drop(&mut self) {
        //IPJToDoHttpRequestDelegate被释放，告诉当前持有IPJToDoHttpRequestDelegate对象的所有权者做相应的处理
        // (self.destroy)(self.user);
        println!("IPJToDoHttpRequestDelegateWrapper -> drop");
    }
}

unsafe impl Send for IPJToDoHttpRequestDelegateWrapper {}
unsafe impl Sync for IPJToDoHttpRequestDelegateWrapper {}
