use crate::common::utils::pj_utils::PJUtils;
use crate::delegates::to_do_tag_delegate::{IPJToDoTagDelegate, IPJToDoTagDelegateWrapper};
use crate::service::to_do_tag_service::{
    PJToDoTagService, insert_todo_tag, delete_todo_tag, update_todo_tag, find_todo_tag_by_id,
    find_todo_tag_by_name, fetch_data,
};
use crate::service::service_impl::to_do_tag_service_impl::createPJToDoTagServiceImpl;
use crate::to_do_tag::to_do_tag::{ToDoTagInsert, ToDoTag, pj_create_ToDoTag, pj_create_ToDoTagInsert};
use crate::common::free_rust_any_object;
use std::ffi::{CStr};
use libc::c_char;
use std::thread;
use std::marker::{Send, Sync};

/*
* cbindgen didn't support Box<dyn PJToDoTagService> type,so I need to use PJToDoTagServiceController to define Box<dyn PJToDoTagService>.
*/
#[repr(C)]
pub struct PJToDoTagServiceController {
    todo_tag_service: Box<dyn PJToDoTagService>,
}

impl PJToDoTagServiceController {
    fn new() -> PJToDoTagServiceController {
        let serviceController = PJToDoTagServiceController {
            todo_tag_service: Box::new(createPJToDoTagServiceImpl()),
        };
        serviceController
    }
}

impl Drop for PJToDoTagServiceController {
    fn drop(&mut self) {
        println!("PJToDoTagServiceController -> drop");
    }
}

unsafe impl Send for PJToDoTagServiceController {}
unsafe impl Sync for PJToDoTagServiceController {}

#[repr(C)]
pub struct PJToDoTagController {
    pub delegate: IPJToDoTagDelegate,
    pub todo_tag_service_controller: *mut PJToDoTagServiceController,
    pub find_result_todo_tag: *mut ToDoTag,
    pub insert_todo_tag: *mut ToDoTagInsert,
    pub todo_tags: *mut Vec<ToDoTag>,
}

unsafe impl Send for PJToDoTagController {}
unsafe impl Sync for PJToDoTagController {}

impl PJToDoTagController {
    fn new(delegate: IPJToDoTagDelegate) -> PJToDoTagController {
        let c_str_tag = PJUtils::create_cstring_from("");
        let c_str_tag_insert = PJUtils::create_cstring_from("");
        let controller = unsafe {
            PJToDoTagController {
                delegate: delegate,
                todo_tag_service_controller: Box::into_raw(Box::new(
                    PJToDoTagServiceController::new(),
                )),
                find_result_todo_tag: pj_create_ToDoTag(c_str_tag.into_raw()),
                insert_todo_tag: pj_create_ToDoTagInsert(c_str_tag_insert.into_raw()),
                todo_tags: Box::into_raw(Box::new(Vec::new())),
            }
        };
        controller
    }

    /**
     * 添加分类
     */
    pub unsafe fn insert_todo_tag(&mut self, to_do_tag: *mut ToDoTagInsert) {
        assert!(to_do_tag != std::ptr::null_mut());
        let i_delegate = IPJToDoTagDelegateWrapper((&self.delegate) as *const IPJToDoTagDelegate);

        let result = insert_todo_tag(
            &(&(*self.todo_tag_service_controller)).todo_tag_service,
            &(*to_do_tag),
        );

        /*free the old todoTypeInsert before set the new one*/
        free_rust_any_object(self.insert_todo_tag);
        self.insert_todo_tag = to_do_tag;

        match result {
            Ok(_) => {
                (i_delegate.insert_result)(i_delegate.user, true);
            }
            Err(_e) => {
                (i_delegate.insert_result)(i_delegate.user, false);
            }
        }
    }

    pub unsafe fn delete_todo_tag(&self, to_do_tag_id: i32) {
        let i_delegate = IPJToDoTagDelegateWrapper((&self.delegate) as *const IPJToDoTagDelegate);

        let result = delete_todo_tag(
            &(&(*self.todo_tag_service_controller)).todo_tag_service,
            to_do_tag_id,
        );

        match result {
            Ok(_) => {
                (i_delegate.delete_result)(i_delegate.user, true);
            }
            Err(_e) => {
                (i_delegate.delete_result)(i_delegate.user, false);
            }
        }
    }

    pub unsafe fn update_todo_tag(&self, to_do_tag: *const ToDoTag) {
        assert!(to_do_tag != std::ptr::null_mut());

        let i_delegate = IPJToDoTagDelegateWrapper((&self.delegate) as *const IPJToDoTagDelegate);

        let result = update_todo_tag(
            &(&(*self.todo_tag_service_controller)).todo_tag_service,
            &(*to_do_tag),
        );

        match result {
            Ok(_) => {
                (i_delegate.update_result)(i_delegate.user, true);
            }
            Err(_e) => {
                (i_delegate.update_result)(i_delegate.user, false);
            }
        }
    }

    pub unsafe fn find_todo_tag_by_id(&mut self, to_do_tag_id: i32) {
        let i_delegate = IPJToDoTagDelegateWrapper((&self.delegate) as *const IPJToDoTagDelegate);

        let result = find_todo_tag_by_id(
            &(&(*self.todo_tag_service_controller)).todo_tag_service,
            to_do_tag_id,
        );

        match result {
            Ok(to_do_tag) => {
                /*free the old todoTag before set the new one*/
                free_rust_any_object(self.find_result_todo_tag);
                let to_do_tag_ptr = Box::into_raw(Box::new(to_do_tag));
                self.find_result_todo_tag = to_do_tag_ptr;
                (i_delegate.find_byId_result)(i_delegate.user, to_do_tag_ptr, true);
            }
            Err(_e) => {
                let mut to_do_tag = ToDoTag {
                    id: -1,
                    tag_name: "".to_owned(),
                };
                let to_do_tag_ptr = &mut to_do_tag as *mut ToDoTag;
                (i_delegate.find_byId_result)(i_delegate.user, to_do_tag_ptr, false);
            }
        }
    }

    pub unsafe fn find_todo_tag_by_name(&mut self, tag_name: String) {
        let i_delegate = IPJToDoTagDelegateWrapper((&self.delegate) as *const IPJToDoTagDelegate);

        let result = find_todo_tag_by_name(
            &(&(*self.todo_tag_service_controller)).todo_tag_service,
            tag_name,
        );

        match result {
            Ok(to_do_tag) => {
                /*free the old todoTag before set the new one*/
                free_rust_any_object(self.find_result_todo_tag);
                let to_do_tag_ptr = Box::into_raw(Box::new(to_do_tag));
                self.find_result_todo_tag = to_do_tag_ptr;
                (i_delegate.find_byName_result)(i_delegate.user, to_do_tag_ptr, true);
            }
            Err(_e) => {
                let mut to_do_tag = ToDoTag {
                    id: -1,
                    tag_name: "".to_owned(),
                };
                let to_do_tag_ptr = &mut to_do_tag as *mut ToDoTag;
                (i_delegate.find_byName_result)(i_delegate.user, to_do_tag_ptr, false);
            }
        }
    }

    pub unsafe fn fetch_data(&mut self) {
        let i_delegate = IPJToDoTagDelegateWrapper((&self.delegate) as *const IPJToDoTagDelegate);

        let result = fetch_data(&(&(*self.todo_tag_service_controller)).todo_tag_service);

        match result {
            Ok(to_do_tags) => {
                /*free the old todo_tags before set the new one*/
                free_rust_any_object(self.todo_tags);
                self.todo_tags = Box::into_raw(Box::new(to_do_tags));
                (i_delegate.fetch_data_result)(i_delegate.user, true);
            }
            Err(_e) => {
                (i_delegate.fetch_data_result)(i_delegate.user, false);
            }
        }
    }

    pub unsafe fn todo_tag_at_index(&self, index: i32) -> *const ToDoTag {
        let index: usize = index as usize;
        assert!(index <= self.get_count());

        let mut todo_tag: *const ToDoTag = std::ptr::null_mut();
        if self.get_count() > 0 {
            todo_tag = &((*(self.todo_tags))[index]);
        }
        todo_tag
    }

    pub unsafe fn get_count(&self) -> usize {
        if self.todo_tags == std::ptr::null_mut() {
            0
        } else {
            let count = (*(self.todo_tags)).len();
            count
        }
    }
}

impl Drop for PJToDoTagController {
    fn drop(&mut self) {
        //PJToDoTagController被释放，告诉当前持有PJToDoTagDelegate对象的所有权者做相应的处理
        unsafe {
            free_rust_any_object(self.todo_tag_service_controller);
            free_rust_any_object(self.find_result_todo_tag);
            free_rust_any_object(self.insert_todo_tag);
            free_rust_any_object(self.todo_tags);
        }
        println!("PJToDoTagController -> drop");
    }
}

/*** extern "C" ***/

#[no_mangle]
pub extern "C" fn pj_create_PJToDoTagController(
    delegate: IPJToDoTagDelegate,
) -> *mut PJToDoTagController {
    let controller = PJToDoTagController::new(delegate);
    Box::into_raw(Box::new(controller))
}

#[no_mangle]
pub unsafe extern "C" fn pj_insert_todo_tag(
    ptr: *mut PJToDoTagController,
    toDoTag: *mut ToDoTagInsert,
) {
    if ptr == std::ptr::null_mut() || toDoTag == std::ptr::null_mut() {
        pj_error!("ptr or toDoTag: *mut insertToDoTag is null!");
        assert!(ptr != std::ptr::null_mut() && toDoTag != std::ptr::null_mut());
    }

    let controler = &mut *ptr;
    let toDoTag = &mut *toDoTag;

    thread::spawn(move || {
        println!("insertToDoTag thread::spawn");
        controler.insert_todo_tag(toDoTag);
    });
}

#[no_mangle]
pub unsafe extern "C" fn pj_delete_todo_tag(ptr: *mut PJToDoTagController, toDoTagId: i32) {
    if ptr == std::ptr::null_mut() {
        pj_error!("ptr: *mut deleteToDoTag is null!");
        assert!(ptr != std::ptr::null_mut());
    }

    let controler = &mut *ptr;

    thread::spawn(move || {
        println!("insertToDoTag thread::spawn");
        controler.delete_todo_tag(toDoTagId);
    });
}

#[no_mangle]
pub unsafe extern "C" fn pj_update_todo_tag(
    ptr: *mut PJToDoTagController,
    toDoTag: *const ToDoTag,
) {
    if ptr == std::ptr::null_mut() || toDoTag == std::ptr::null_mut() {
        pj_error!("ptr or toDoTag: *mut updateToDoTag is null!");
        assert!(ptr != std::ptr::null_mut() && toDoTag != std::ptr::null_mut());
    }

    let controler = &mut *ptr;
    let toDoTag = &*toDoTag;

    thread::spawn(move || {
        println!("insertToDoTag thread::spawn");
        controler.update_todo_tag(toDoTag);
    });
}

#[no_mangle]
pub unsafe extern "C" fn pj_find_todo_tag(ptr: *mut PJToDoTagController, toDoTagId: i32) {
    if ptr == std::ptr::null_mut() {
        pj_error!("ptr: *mut findToDoTag is null!");
        assert!(ptr != std::ptr::null_mut());
    }

    let controler = &mut *ptr;

    thread::spawn(move || {
        println!("insertToDoTag thread::spawn");
        controler.find_todo_tag_by_id(toDoTagId);
    });
}

#[no_mangle]
pub unsafe extern "C" fn pj_find_todo_tag_by_name(
    ptr: *mut PJToDoTagController,
    tag_name: *const c_char,
) {
    if ptr == std::ptr::null_mut() || tag_name == std::ptr::null_mut() {
        pj_error!("ptr or typeName: *mut findToDoTagByName is null!");
        assert!(ptr != std::ptr::null_mut() && tag_name != std::ptr::null_mut());
    }

    let controler = &mut *ptr;
    let tag_name = CStr::from_ptr(tag_name).to_string_lossy().into_owned();

    thread::spawn(move || {
        println!("insertToDoTag thread::spawn");
        controler.find_todo_tag_by_name(tag_name);
    });
}

#[no_mangle]
pub unsafe extern "C" fn pj_fetch_todo_tag_data(ptr: *mut PJToDoTagController) {
    if ptr == std::ptr::null_mut() {
        pj_error!("ptr or toDoTag: *mut fetchData is null!");
        assert!(ptr != std::ptr::null_mut());
    }
    let controler = &mut *ptr;
    controler.fetch_data()
}

#[no_mangle]
pub unsafe extern "C" fn pj_todo_tag_at_index(
    ptr: *const PJToDoTagController,
    index: i32,
) -> *const ToDoTag {
    if ptr == std::ptr::null_mut() {
        pj_error!("ptr or toDoTag: *mut todoTagAtIndex is null!");
        assert!(ptr != std::ptr::null_mut());
    }
    let controler = &*ptr;
    controler.todo_tag_at_index(index)
}

#[no_mangle]
pub unsafe extern "C" fn pj_get_todo_tag_count(ptr: *const PJToDoTagController) -> i32 {
    if ptr == std::ptr::null_mut() {
        pj_error!("ptr or toDoTag: *mut getToDoTagCount is null!");
        assert!(ptr != std::ptr::null_mut());
    }
    let controler = &*ptr;
    controler.get_count() as i32
}

#[no_mangle]
pub unsafe extern "C" fn pj_free_rust_PJToDoTagController(ptr: *mut PJToDoTagController) {
    if ptr != std::ptr::null_mut() {
        let _ = Box::from_raw(ptr); //unsafe
    };
}
