#![feature(rustc_private)]
extern crate libc;
use libc::c_char;
use std::ffi::{CStr, CString};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

// cargo lipo --verbose --targets=aarch64-apple-ios,x86_64-apple-ios
/*
* #[no_mangle]: 避免名字被编译器重整
* extern: extern用于FFI中的两个不同上下文环境中。用于声明Rust代码可以被外部调用的函数接口。
*/
#[no_mangle]
pub extern "C" fn addition(a: i32, b: i32) -> i32 {
    unsafe {
        test_pal_from_Swift();
    }
    a + b
}

#[no_mangle]
pub unsafe extern "C" fn str_from_rust() -> *mut c_char {
    let song = String::from("😘😘😘😘😘😘");
    let c_str_song = CString::new(song).unwrap();
    c_str_song.into_raw()
}

pub struct Person {
    pub age: i32,
    pub name: String,
}

impl Person {
    fn new() -> Person {
        Person {
            age: 20,
            name: String::from("Zoey")
        }
    }

    fn get_age(&self) -> i32 {
        self.age
    }

    fn get_name(&self) -> &str {
        self.name.as_str()
    }
}


/* Delegate */

use std::ops::Deref;
use libc::{c_void};

#[repr(C)]
pub struct IPersonUIControllerDelegate {
    pub user: *mut c_void, //当前持有IPersonUIControllerDelegate对象的所有权者
    //释放内存回调，告诉当前持有IPersonUIControllerDelegate对象的所有权者做相应的处理
    pub destroy: extern "C" fn(user: *mut c_void),
    pub did_update: extern "C" fn(user: *mut c_void, isSuccess: bool),
}

impl Drop for IPersonUIControllerDelegate {
    fn drop(&mut self) {
        //IPersonUIControllerDelegate
        println!("IPJToDoDelegate -> drop");
    }
}

//该类的作用是当IPersonUIControllerDelegate被销毁时能够释放内存
pub struct IPersonUIControllerDelegateWrapper(pub *const IPersonUIControllerDelegate);

impl Deref for IPersonUIControllerDelegateWrapper {
    type Target = IPersonUIControllerDelegate;

    fn deref(&self) -> &IPersonUIControllerDelegate {
        unsafe { &(*self.0) }
    }
}

impl Drop for IPersonUIControllerDelegateWrapper {
    fn drop(&mut self) {
        //IPersonUIControllerDelegate被释放，告诉当前持有IPersonUIControllerDelegate对象的所有权者做相应的处理
        (self.destroy)(self.user);
        println!("IPersonUIControllerDelegateWrapper -> drop");
    }
}

/* UIController */

/*The create and free are both in Rust. only free PJToDoController in Swift.*/
#[repr(C)]
pub struct IPersonUIController {
    pub delegate: IPersonUIControllerDelegate,
    pub person: Person //ViewModel
}

impl IPersonUIController {
    fn new(delegate: IPersonUIControllerDelegate) -> IPersonUIController {
        let controller = IPersonUIController {
            delegate: delegate,
            person: Person::new()
        };
        controller
    }

    fn update_person(&mut self, new_age: i32, new_name: String) {
        self.person.age = new_age;
        self.person.name = new_name;
        let i_delegate =
        IPersonUIControllerDelegateWrapper((&self.delegate) as *const IPersonUIControllerDelegate);
        (i_delegate.did_update)(i_delegate.user, true);
    }
}

#[no_mangle]
pub extern "C" fn create_person_uicontroller(delegate: IPersonUIControllerDelegate) -> *mut IPersonUIController {
    let controller = IPersonUIController::new(delegate);
    Box::into_raw(Box::new(controller))
}

#[no_mangle]
pub unsafe extern fn free_person_uicontroller(ptr: *mut IPersonUIController) {
    if ptr.is_null() { return };
    Box::from_raw(ptr);//unsafe
}

#[no_mangle]
pub unsafe extern "C" fn update_person(ptr: *mut IPersonUIController, new_age: i32, new_name: *const c_char) {
    assert!(ptr != std::ptr::null_mut());
    let controler = &mut *ptr;
    let new_name = CStr::from_ptr(new_name).to_string_lossy().into_owned();
    controler.update_person(new_age, new_name);
}

#[no_mangle]
pub unsafe extern fn get_person_age(ptr: *const IPersonUIController) -> i32 {
    assert!(ptr != std::ptr::null_mut());
    let controler = &*ptr;
    controler.person.get_age()
}

#[no_mangle]
pub unsafe extern fn get_person_name(ptr: *const IPersonUIController) -> *mut c_char {
    assert!(ptr != std::ptr::null_mut());
    let controler = &*ptr;
    let name = controler.person.get_name();
    let c_name = CString::new(name).unwrap();//unsafe
    c_name.into_raw()
}


#[link(name = "PJToDoCoreLibPAL")]

extern "C" {
    #[link_name = "\u{1}_test_pal_from_Swift"]
    pub fn test_pal_from_Swift();
}