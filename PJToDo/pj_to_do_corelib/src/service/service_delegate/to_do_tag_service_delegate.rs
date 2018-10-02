use std::ops::Deref;
use libc::{c_void, c_char};

#[repr(C)]
pub struct PJToDoTypeServiceDelegate {
    user: *mut c_void,//当前持有PJToDoTypeServiceDelegate对象的所有权者
    //释放内存回调，告诉当前持有PJToDoTypeServiceDelegate对象的所有权者做相应的处理
    destroy: extern fn(user: *mut c_void),
}

impl Drop for PJToDoTypeServiceDelegate {
    fn drop(&mut self) {
        //PJToDoTypeServiceDelegate被释放，告诉当前持有PJToDoTypeServiceDelegate对象的所有权者做相应的处理
        (self.destroy)(self.user);
        println!("PJToDoTypeServiceDelegate -> drop");
    }
}

//该类的作用是当PJToDoTypeServiceDelegate被销毁时能够释放内存
pub struct PJToDoTypeServiceDelegateWrapper<'a>(&'a PJToDoTypeServiceDelegate);

impl<'a> Deref for PJToDoTypeServiceDelegateWrapper<'a> {
    type Target = PJToDoTypeServiceDelegate;

    fn deref(&self) -> &PJToDoTypeServiceDelegate {
        &self.0
    }
}

impl<'a> Drop for PJToDoTypeServiceDelegateWrapper<'a> {
    fn drop(&mut self) {
        //PJToDoTypeServiceDelegate被释放，告诉当前持有PJToDoTypeServiceDelegate对象的所有权者做相应的处理
        (self.destroy)(self.user);
        println!("PJToDoTypeServiceDelegateWrapper -> drop");
    }
}