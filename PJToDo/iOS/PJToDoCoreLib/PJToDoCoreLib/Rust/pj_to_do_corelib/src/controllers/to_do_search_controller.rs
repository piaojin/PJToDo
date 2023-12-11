use delegates::to_do_search_delegate::{IPJToDoSearchDelegate, IPJToDoSearchDelegateWrapper};
use service::to_do_service::{PJToDoService, find_todo_by_title, find_todo_like_title};
use service::service_impl::to_do_service_impl::{createPJToDoServiceImpl};
use to_do::to_do::{ToDoQuery};
use to_do_type::to_do_type::ToDoType;
use to_do_tag::to_do_tag::ToDoTag;
use common::{free_rust_any_object};
#[allow(unused_imports)]
use common::pj_logger::PJLogger;
use std::ffi::{CStr};
use libc::{c_char};
use std::thread;
use std::marker::{Send, Sync};
use std::ptr;
use common::pj_serialize::PJSerdeDeserialize;
use service::to_do_type_service::PJToDoTypeService;
use service::to_do_tag_service::PJToDoTagService;
use service::service_impl::to_do_type_service_impl::{createPJToDoTypeServiceImpl};
use service::service_impl::to_do_tag_service_impl::{createPJToDoTagServiceImpl};

/*
* cbindgen didn't support Box<dyn PJToDoService> type,so I need to use PJToDoSearchServiceController to define Box<dyn PJToDoService>.
*/
#[repr(C)]
pub struct PJToDoSearchServiceController {
    todo_service: Box<dyn PJToDoService>,
}

impl PJToDoSearchServiceController {
    fn new() -> PJToDoSearchServiceController {
        let serviceController = PJToDoSearchServiceController {
            todo_service: Box::new(createPJToDoServiceImpl()),
        };
        serviceController
    }
}

impl Drop for PJToDoSearchServiceController {
    fn drop(&mut self) {
        println!("PJToDoSearchServiceController -> drop");
    }
}

unsafe impl Send for PJToDoSearchServiceController {}
unsafe impl Sync for PJToDoSearchServiceController {}

/*The create and free are both in Rust. only free PJToDoSearchController in Swift.*/
#[repr(C)]
pub struct PJToDoSearchController {
    pub delegate: IPJToDoSearchDelegate,
    pub todo_search_service_controller: *mut PJToDoSearchServiceController,
    pub todos: *mut Vec<ToDoQuery>, // all todos without order by state
    pub todo_types: *mut Vec<ToDoType>, // all todo types
    pub todo_tags: *mut Vec<ToDoTag>, // all todo tag
    pub find_result_todo: *mut ToDoQuery,
}

unsafe impl Send for PJToDoSearchController {}
unsafe impl Sync for PJToDoSearchController {}

impl PJToDoSearchController {
    fn new(delegate: IPJToDoSearchDelegate) -> PJToDoSearchController {
        let mut controller = PJToDoSearchController {
            delegate: delegate,
            todo_search_service_controller: Box::into_raw(Box::new(
                PJToDoSearchServiceController::new(),
            )),
            todos: ptr::null_mut(),
            todo_types: ptr::null_mut(),
            todo_tags: ptr::null_mut(),
            find_result_todo: ptr::null_mut(),
        };
        unsafe {
            controller.prepare_datas();
        }
        controller
    }

    pub unsafe fn find_todo_by_title(&mut self, title: String) {
        let i_delegate =
            IPJToDoSearchDelegateWrapper((&self.delegate) as *const IPJToDoSearchDelegate);

        let result = find_todo_by_title(
            &(&(*self.todo_search_service_controller)).todo_service,
            title,
        );

        match result {
            Ok(to_do) => {
                /*free the old todo before set the new one*/
                free_rust_any_object(self.find_result_todo);
                let to_do_ptr = Box::into_raw(Box::new(to_do));
                self.find_result_todo = to_do_ptr;
                (i_delegate.find_byTitle_result)(i_delegate.user, to_do_ptr, true);
            }
            Err(_e) => {
                let mut to_do = ToDoQuery::new();
                let to_do_ptr = &mut to_do as *mut ToDoQuery;
                (i_delegate.find_byTitle_result)(i_delegate.user, to_do_ptr, false);
            }
        }
    }

    pub unsafe fn prepare_datas(&mut self) -> bool {
        let mut is_prepare_success: bool = true;
        let todo_type_result = self.fetch_types();
        match todo_type_result {
            Ok(todo_types) => {
                /*free the old todo before set the new one*/
                free_rust_any_object(self.todo_types);
                self.todo_types = Box::into_raw(Box::new(todo_types));
            }
            Err(_e) => {
                is_prepare_success = false;
            }
        }

        if !is_prepare_success {
            ()
        }

        let todo_tag_result = self.fetch_tags();
        match todo_tag_result {
            Ok(todo_tags) => {
                /*free the old todo before set the new one*/
                free_rust_any_object(self.todo_tags);
                self.todo_tags = Box::into_raw(Box::new(todo_tags));
            }
            Err(_e) => {
                is_prepare_success = false;
            }
        }

        is_prepare_success
    }

    pub unsafe fn fetch_types(&mut self) -> Result<Vec<ToDoType>, diesel::result::Error> {
        let todo_type_service: Box<dyn PJToDoTypeService> = Box::new(createPJToDoTypeServiceImpl());
        let todo_type_result = todo_type_service.fetch_data();
        match todo_type_result {
            Ok(todo_types) => {
                pj_info!("fetch_types success!");
                Ok(todo_types)
            }
            Err(e) => {
                pj_error!("fetch_types faild!");
                Err(e)
            }
        }
    }

    pub unsafe fn fetch_tags(&mut self) -> Result<Vec<ToDoTag>, diesel::result::Error> {
        let todo_tag_service: Box<dyn PJToDoTagService> = Box::new(createPJToDoTagServiceImpl());
        let todo_tag_result = todo_tag_service.fetch_data();
        match todo_tag_result {
            Ok(todo_tags) => {
                pj_info!("fetch_tags success!");
                Ok(todo_tags)
            }
            Err(e) => {
                pj_error!("fetch_tags faild!");
                Err(e)
            }
        }
    }

    pub unsafe fn find_todo_like_title(&mut self, title: String) {
        let i_delegate =
            IPJToDoSearchDelegateWrapper((&self.delegate) as *const IPJToDoSearchDelegate);

        let result = find_todo_like_title(
            &(&(*self.todo_search_service_controller)).todo_service,
            title,
        );

        match result {
            Ok(like_title_result_todos) => {
                /*free the old todos before set the new one*/
                free_rust_any_object(self.todos);
                self.todos = Box::into_raw(Box::new(like_title_result_todos));
                (i_delegate.find_byLike_result)(i_delegate.user, true);
            }
            Err(_e) => {
                (i_delegate.find_byLike_result)(i_delegate.user, false);
            }
        }
    }

    pub unsafe fn todo_at_index(&self, index: i32) -> *const ToDoQuery {
        let index: usize = index as usize;

        assert!(index <= self.get_count());

        let mut todo: *const ToDoQuery = std::ptr::null_mut();
        if self.get_count() > 0 {
            todo = &((*(self.todos))[index]);
        }
        todo
    }

    pub unsafe fn get_count(&self) -> usize {
        if self.todos == std::ptr::null_mut() {
            0
        } else {
            let count = (*(self.todos)).len();
            count
        }
    }

    pub unsafe fn todo_type_with_id(&self, type_id: i32) -> *const ToDoType {
        if self.todo_types != std::ptr::null_mut() {
            let result = (*self.todo_types)
                .iter()
                .find(|ref mut todo_type| todo_type.id == type_id);
            match result {
                Some(todo_type) => todo_type,
                None => {
                    println!("todo_type_with_id didn't find!");
                    std::ptr::null()
                }
            }
        } else {
            std::ptr::null()
        }
    }

    pub unsafe fn todo_tag_with_id(&self, tag_id: i32) -> *const ToDoTag {
        if self.todo_tags != std::ptr::null_mut() {
            let result = (*self.todo_tags)
                .iter()
                .find(|ref mut todo_tag| todo_tag.id == tag_id);
            match result {
                Some(todo_tag) => todo_tag,
                None => {
                    println!("todo_tag_with_id tag_id {} didn't find!", tag_id);
                    std::ptr::null()
                }
            }
        } else {
            std::ptr::null()
        }
    }
}

impl Drop for PJToDoSearchController {
    fn drop(&mut self) {
        //PJToDoSearchController被释放，告诉当前持有PJToDoDelegate对象的所有权者做相应的处理
        unsafe {
            free_rust_any_object(self.todo_search_service_controller);
            free_rust_any_object(self.todos);
            free_rust_any_object(self.todo_types);
            free_rust_any_object(self.todo_tags);
            free_rust_any_object(self.find_result_todo);
        }
        println!("PJToDoSearchController -> drop");
    }
}

// /*** extern "C" ***/
#[no_mangle]
pub extern "C" fn pj_create_PJToDoSearchController(
    delegate: IPJToDoSearchDelegate,
) -> *mut PJToDoSearchController {
    let controller = PJToDoSearchController::new(delegate);
    Box::into_raw(Box::new(controller))
}

#[no_mangle]
pub unsafe extern "C" fn pj_find_todo_by_title(
    ptr: *mut PJToDoSearchController,
    title: *const c_char,
) {
    if ptr == std::ptr::null_mut() || title == std::ptr::null_mut() {
        pj_error!("ptr or title: *mut findToDoByTitle is null!");
        assert!(ptr != std::ptr::null_mut() && title != std::ptr::null_mut());
    }

    let controler = &mut *ptr;
    let title = CStr::from_ptr(title).to_string_lossy().into_owned();

    thread::spawn(move || {
        println!("insertToDo thread::spawn");
        controler.find_todo_by_title(title);
    });
}

#[no_mangle]
pub unsafe extern "C" fn pj_find_todo_like_title(
    ptr: *mut PJToDoSearchController,
    title: *const c_char,
) {
    if ptr == std::ptr::null_mut() || title == std::ptr::null_mut() {
        pj_error!("ptr or title: *mut findToDoLikeTitle is null!");
        assert!(ptr != std::ptr::null_mut() && title != std::ptr::null_mut());
    }

    let controler = &mut *ptr;
    let title = CStr::from_ptr(title).to_string_lossy().into_owned();

    thread::spawn(move || {
        println!("insertToDo thread::spawn");
        controler.find_todo_like_title(title)
    });
}

#[no_mangle]
pub unsafe extern "C" fn pj_search_todo_result_at_index(
    ptr: *const PJToDoSearchController,
    index: i32,
) -> *const ToDoQuery {
    if ptr == std::ptr::null_mut() {
        pj_error!("ptr or toDo: *mut todoAtIndex is null!");
        assert!(ptr != std::ptr::null_mut());
    }
    let controler = &*ptr;
    controler.todo_at_index(index)
}

#[no_mangle]
pub unsafe extern "C" fn pj_search_todo_result_count(ptr: *const PJToDoSearchController) -> i32 {
    if ptr == std::ptr::null_mut() {
        pj_error!("ptr or toDo: *mut getToDoCount is null!");
        assert!(ptr != std::ptr::null_mut());
    }
    let controler = &*ptr;
    controler.get_count() as i32
}

#[no_mangle]
pub unsafe extern "C" fn pj_get_search_todo_type_with_id(
    ptr: *const PJToDoSearchController,
    type_id: i32,
) -> *const ToDoType {
    if ptr == std::ptr::null_mut() {
        pj_error!("ptr or toDo: *mut todo_type_with_id is null!");
        assert!(ptr != std::ptr::null_mut());
    }
    let controler = &*ptr;
    controler.todo_type_with_id(type_id)
}

#[no_mangle]
pub unsafe extern "C" fn pj_get_search_todo_tag_with_id(
    ptr: *const PJToDoSearchController,
    tag_id: i32,
) -> *const ToDoTag {
    if ptr == std::ptr::null_mut() {
        pj_error!("ptr or toDo: *mut todo_tag_with_id is null!");
        assert!(ptr != std::ptr::null_mut());
    }
    let controler = &*ptr;
    controler.todo_tag_with_id(tag_id)
}

#[no_mangle]
pub unsafe extern "C" fn pj_free_rust_PJToDoSearchController(ptr: *mut PJToDoSearchController) {
    if ptr != std::ptr::null_mut() {
        let _ = Box::from_raw(ptr); //unsafe
    }
}
