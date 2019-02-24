use std::ops::Deref;
use libc::{c_void};
use std::marker::{Send, Sync};

#[derive(Clone)]
#[repr(C)]
pub struct IPJToDoDownLoadFileDelegate {
    pub user: *mut c_void, //当前持有IPJToDoDownLoadFileDelegate对象的所有权者
    //释放内存回调，告诉当前持有IPJToDoDownLoadFileDelegate对象的所有权者做相应的处理
    pub destroy: extern "C" fn(user: *mut c_void),
    pub request_result: extern "C" fn(user: *mut c_void, statusCode: u16, is_success: bool),
}

impl Drop for IPJToDoDownLoadFileDelegate {
    fn drop(&mut self) {
        //IPJToDoDownLoadFileDelegate被释放，告诉当前持有IPJToDoDownLoadFileDelegate对象的所有权者做相应的处理
        println!("IPJToDoDownLoadFileDelegate -> drop");
    }
}

//该类的作用是当IPJToDoDownLoadFileDelegate被销毁时能够释放内存
#[derive(Clone)]
pub struct IPJToDoDownLoadFileDelegateWrapper(pub IPJToDoDownLoadFileDelegate);

impl Deref for IPJToDoDownLoadFileDelegateWrapper {
    type Target = IPJToDoDownLoadFileDelegate;

    fn deref(&self) -> &IPJToDoDownLoadFileDelegate {
        &(self.0)
    }
}

impl Drop for IPJToDoDownLoadFileDelegateWrapper {
    fn drop(&mut self) {
        //IPJToDoDownLoadFileDelegate被释放，告诉当前持有IPJToDoDownLoadFileDelegate对象的所有权者做相应的处理
        // (self.destroy)(self.user);
        println!("IPJToDoDownLoadFileDelegateWrapper -> drop");
    }
}

unsafe impl Send for IPJToDoDownLoadFileDelegateWrapper {}
unsafe impl Sync for IPJToDoDownLoadFileDelegateWrapper {}
