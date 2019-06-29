use delegates::to_do_delegate::{IPJToDoDelegate, IPJToDoDelegateWrapper};
use service::to_do_service::{
    PJToDoService, insert_todo, delete_todo, update_todo, find_todo_by_id,
    find_todo_date_future_day_more_than, fetch_todos_order_by_state, update_overdue_todos,
};
use service::service_impl::to_do_service_impl::{createPJToDoServiceImpl};
use to_do::to_do::{ToDoInsert, ToDoQuery};
use to_do_type::to_do_type::ToDoType;
use to_do_tag::to_do_tag::ToDoTag;
use common::{free_rust_any_object};
#[allow(unused_imports)]
use common::pj_logger::PJLogger;
use std::thread;
use std::marker::{Send, Sync};
use common::pj_serialize::PJSerdeDeserialize;
use service::to_do_type_service::PJToDoTypeService;
use service::to_do_tag_service::PJToDoTagService;
use service::service_impl::to_do_type_service_impl::{createPJToDoTypeServiceImpl};
use service::service_impl::to_do_tag_service_impl::{createPJToDoTagServiceImpl};
use std::ptr;
use libc::c_char;
use std::ffi::{CString};

/*
* cbindgen didn't support Box<dyn PJToDoService> type,so I need to use PJToDoServiceController to define Box<dyn PJToDoService>.
*/
#[repr(C)]
pub struct PJToDoServiceController {
    todo_service: Box<dyn PJToDoService>,
}

impl PJToDoServiceController {
    fn new() -> PJToDoServiceController {
        let serviceController = PJToDoServiceController {
            todo_service: Box::new(createPJToDoServiceImpl()),
        };
        serviceController
    }
}

impl Drop for PJToDoServiceController {
    fn drop(&mut self) {
        println!("PJToDoServiceController -> drop");
    }
}

unsafe impl Send for PJToDoServiceController {}
unsafe impl Sync for PJToDoServiceController {}

/*The create and free are both in Rust. only free PJToDoController in Swift.*/
#[repr(C)]
pub struct PJToDoController {
    pub delegate: IPJToDoDelegate,
    pub todo_service_controller: *mut PJToDoServiceController,
    pub find_result_todo: *mut ToDoQuery, //find by id or by title will save in find_result_todo
    pub insert_todo: *mut ToDoInsert,
    pub todos: *mut Vec<Vec<ToDoQuery>>, // all todos without order by state
    pub todo_types: *mut Vec<ToDoType>,  // all todo types
    pub todo_tags: *mut Vec<ToDoTag>,    // all todo tag
    pub sectionTitles: *const Vec<String>,
}

unsafe impl Send for PJToDoController {}
unsafe impl Sync for PJToDoController {}

impl PJToDoController {
    fn new(delegate: IPJToDoDelegate) -> PJToDoController {
        let controller = PJToDoController {
            delegate: delegate,
            todo_service_controller: Box::into_raw(Box::new(PJToDoServiceController::new())),
            find_result_todo: ptr::null_mut(),
            insert_todo: ptr::null_mut(),
            todos: ptr::null_mut(),
            todo_types: ptr::null_mut(),
            todo_tags: ptr::null_mut(),
            sectionTitles: Box::into_raw(Box::new(vec![
                "即将过期".to_string(),
                "进行中".to_string(),
                "待定".to_string(),
                "完成".to_string(),
                "已经过期".to_string(),
            ])),
        };
        controller
    }

    pub unsafe fn insert_todo(&mut self, to_do: *mut ToDoInsert) {
        pj_info!("insert_todo: {}", (*to_do).title);
        assert!(to_do != std::ptr::null_mut());
        let i_delegate = IPJToDoDelegateWrapper((&self.delegate) as *const IPJToDoDelegate);

        let result = insert_todo(&(&(*self.todo_service_controller)).todo_service, &(*to_do));

        /*free the old todoInsert before set the new one*/
        free_rust_any_object(self.insert_todo);
        self.insert_todo = to_do;

        match result {
            Ok(_) => {
                (i_delegate.insert_result)(i_delegate.user, true);
            }
            Err(_e) => {
                (i_delegate.insert_result)(i_delegate.user, false);
            }
        }
    }

    pub unsafe fn delete_todo(&self, section: i32, index: i32, to_do_id: i32) {
        let i_delegate = IPJToDoDelegateWrapper((&self.delegate) as *const IPJToDoDelegate);

        let result = delete_todo(&(&(*self.todo_service_controller)).todo_service, to_do_id);

        match result {
            Ok(_) => {
                let todo: *const ToDoQuery = self.todo_at_section(section, index);
                if (*todo).id == to_do_id {
                    ((*(self.todos))[section as usize]).remove(index as usize);
                    (i_delegate.delete_result)(i_delegate.user, true);
                } else {
                    pj_info!("to_do_id didn't match!");
                    (i_delegate.delete_result)(i_delegate.user, false);
                }
            }
            Err(_e) => {
                (i_delegate.delete_result)(i_delegate.user, false);
            }
        }
    }

    pub unsafe fn delete_todo_by_id(&self, to_do_id: i32) {
        let i_delegate = IPJToDoDelegateWrapper((&self.delegate) as *const IPJToDoDelegate);

        let result = delete_todo(&(&(*self.todo_service_controller)).todo_service, to_do_id);

        match result {
            Ok(_) => {
                (i_delegate.delete_result)(i_delegate.user, true);
            }
            Err(_e) => {
                (i_delegate.delete_result)(i_delegate.user, false);
            }
        }
    }

    pub unsafe fn update_todo(&self, to_do: *const ToDoQuery) {
        assert!(to_do != std::ptr::null_mut());

        let i_delegate = IPJToDoDelegateWrapper((&self.delegate) as *const IPJToDoDelegate);

        let result = update_todo(&(&(*self.todo_service_controller)).todo_service, &(*to_do));

        match result {
            Ok(_) => {
                (i_delegate.update_result)(i_delegate.user, true);
            }
            Err(_e) => {
                (i_delegate.update_result)(i_delegate.user, false);
            }
        }
    }

    pub unsafe fn find_todo_by_id(&mut self, to_do_id: i32) {
        let i_delegate = IPJToDoDelegateWrapper((&self.delegate) as *const IPJToDoDelegate);

        let result = find_todo_by_id(&(&(*self.todo_service_controller)).todo_service, to_do_id);

        match result {
            Ok(to_do) => {
                /*free the old todo before set the new one*/
                free_rust_any_object(self.find_result_todo);
                let to_do_ptr = Box::into_raw(Box::new(to_do));
                self.find_result_todo = to_do_ptr;
                (i_delegate.find_byId_result)(i_delegate.user, to_do_ptr, true);
            }
            Err(_e) => {
                let mut to_do = ToDoQuery::new();
                let to_do_ptr = &mut to_do as *mut ToDoQuery;
                (i_delegate.find_byId_result)(i_delegate.user, to_do_ptr, false);
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

        if !is_prepare_success {
            ()
        }

        let mut all_datas: Vec<Vec<ToDoQuery>> = Vec::new();

        let will_due_todos_result =
            self.fecth_todo_will_due_with_in_future_days("now".to_owned(), 7);
        match will_due_todos_result {
            Ok(todos) => {
                /*free the old todos before set the new one*/
                all_datas.push(todos);
            }
            Err(_e) => {
                is_prepare_success = false;
            }
        }

        if !is_prepare_success {
            ()
        }

        let todos_order_by_state_result = self.fetch_todos_order_by_state();
        match todos_order_by_state_result {
            Ok(mut todos) => {
                /*free the old todos before set the new one*/
                free_rust_any_object(self.todos);
                all_datas.append(&mut todos);
                self.todos = Box::into_raw(Box::new(all_datas));
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

    pub unsafe fn fetch_data(&mut self) {
        let i_delegate = IPJToDoDelegateWrapper((&self.delegate) as *const IPJToDoDelegate);

        /*before fetch all todo datas we need to fecth all types and tags*/
        let is_prepare_datas_success = self.prepare_datas();

        (i_delegate.fetch_data_result)(i_delegate.user, is_prepare_datas_success);
    }

    pub unsafe fn fetch_todos_order_by_state(
        &mut self,
    ) -> Result<Vec<Vec<ToDoQuery>>, diesel::result::Error> {
        let result = fetch_todos_order_by_state(&(&(*self.todo_service_controller)).todo_service);

        match result {
            Ok(todos) => {
                pj_info!("fetch_todos_order_by_state success!");
                Ok(todos)
            }
            Err(e) => {
                pj_error!("fetch_todos_order_by_state faild!");
                Err(e)
            }
        }
    }

    pub unsafe fn fecth_todo_will_due_with_in_future_days(
        &mut self,
        from_day: String,
        comparison_days: i32,
    ) -> Result<Vec<ToDoQuery>, diesel::result::Error> {
        let result = find_todo_date_future_day_more_than(
            &(&(*self.todo_service_controller)).todo_service,
            from_day,
            comparison_days,
        );

        match result {
            Ok(todos) => {
                pj_error!("fecth_todo_will_due_with_in_future_days success!");
                Ok(todos)
            }
            Err(e) => {
                pj_error!("fecth_todo_will_due_with_in_future_days faild!");
                Err(e)
            }
        }
    }

    pub unsafe fn update_overdue_todos(&self) {
        let i_delegate = IPJToDoDelegateWrapper((&self.delegate) as *const IPJToDoDelegate);
        let result = update_overdue_todos(&(&(*self.todo_service_controller)).todo_service);
        match result {
            Ok(_) => {
                pj_info!("update_overdue_todos success!");
                (i_delegate.update_overdue_todos)(i_delegate.user, true);
            }
            Err(e) => {
                pj_error!("❌❌update_overdue_todos faild: {:?}!", e);
                (i_delegate.update_overdue_todos)(i_delegate.user, false);
            }
        }
    }

    pub unsafe fn todo_title_at_section(&self, section: i32) -> *mut c_char {
        let section: usize = section as usize;

        assert!(section <= (*(self.sectionTitles)).len());

        let title = CString::new((*(self.sectionTitles))[section].clone()).unwrap(); //unsafe
        title.into_raw()
    }

    pub unsafe fn todo_at_section(&self, section: i32, index: i32) -> *const ToDoQuery {
        let section: usize = section as usize;
        let index: usize = index as usize;

        assert!(section <= self.get_count_of_sections());
        assert!(index <= self.get_count_at_section(section as i32));

        let mut todo: *const ToDoQuery = std::ptr::null_mut();
        if self.get_count_of_sections() > 0 {
            let section_todos = &(*(self.todos))[section];
            if self.get_count_at_section(section as i32) > 0 {
                todo = &(section_todos[index]);
            }
        }
        todo
    }

    pub unsafe fn get_count_of_sections(&self) -> usize {
        let is_null: bool = self.todos == std::ptr::null_mut();
        println!("self.todos == null: {}", is_null);
        if is_null {
            0
        } else {
            let sections = (*(self.todos)).len();
            sections
        }
    }

    pub unsafe fn get_count_at_section(&self, section: i32) -> usize {
        if self.todos == std::ptr::null_mut() {
            0
        } else {
            let section: usize = section as usize;
            /*let sections = (*(self.todos)).len(); here may crash*/
            assert!(section <= self.get_count_of_sections());

            let todos = &(*(self.todos))[section];
            let count = todos.len();
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

impl Drop for PJToDoController {
    fn drop(&mut self) {
        //PJToDoController被释放，告诉当前持有PJToDoDelegate对象的所有权者做相应的处理
        unsafe {
            free_rust_any_object(self.todo_service_controller);
            free_rust_any_object(self.find_result_todo);
            free_rust_any_object(self.insert_todo);
            free_rust_any_object(self.todos);
            free_rust_any_object(self.todo_types);
            free_rust_any_object(self.todo_tags);
        }
        println!("PJToDoController -> drop");
    }
}

// /*** extern "C" ***/
#[no_mangle]
pub extern "C" fn pj_create_PJToDoController(delegate: IPJToDoDelegate) -> *mut PJToDoController {
    let controller = PJToDoController::new(delegate);
    Box::into_raw(Box::new(controller))
}

#[no_mangle]
pub unsafe extern "C" fn pj_insert_todo(ptr: *mut PJToDoController, toDo: *mut ToDoInsert) {
    if ptr == std::ptr::null_mut() || toDo == std::ptr::null_mut() {
        pj_error!("ptr or toDo: *mut insertToDo is null!");
        assert!(ptr != std::ptr::null_mut() && toDo != std::ptr::null_mut());
    }

    let controler = &mut *ptr;
    let toDo = &mut *toDo;

    thread::spawn(move || {
        println!("insertToDo thread::spawn");
        controler.insert_todo(toDo);
    });
}

#[no_mangle]
pub unsafe extern "C" fn pj_delete_todo(
    ptr: *mut PJToDoController,
    section: i32,
    index: i32,
    toDoId: i32,
) {
    if ptr == std::ptr::null_mut() {
        pj_error!("ptr: *mut deleteToDo is null!");
        assert!(ptr != std::ptr::null_mut());
    }

    let controler = &mut *ptr;

    thread::spawn(move || {
        println!("insertToDo thread::spawn");
        controler.delete_todo(section, index, toDoId);
    });
}

#[no_mangle]
pub unsafe extern "C" fn pj_delete_todo_by_id(ptr: *mut PJToDoController, toDoId: i32) {
    if ptr == std::ptr::null_mut() {
        pj_error!("ptr: *mut deleteToDo is null!");
        assert!(ptr != std::ptr::null_mut());
    }

    let controler = &mut *ptr;

    thread::spawn(move || {
        println!("insertToDo thread::spawn");
        controler.delete_todo_by_id(toDoId);
    });
}

#[no_mangle]
pub unsafe extern "C" fn pj_update_todo(ptr: *mut PJToDoController, toDo: *const ToDoQuery) {
    if ptr == std::ptr::null_mut() || toDo == std::ptr::null_mut() {
        pj_error!("ptr or toDo: *mut updateToDo is null!");
        assert!(ptr != std::ptr::null_mut() && toDo != std::ptr::null_mut());
    }

    let controler = &mut *ptr;
    let toDo = &*toDo;

    thread::spawn(move || {
        println!("insertToDo thread::spawn");
        controler.update_todo(toDo);
    });
}

#[no_mangle]
pub unsafe extern "C" fn pj_find_todo(ptr: *mut PJToDoController, toDoId: i32) {
    if ptr == std::ptr::null_mut() {
        pj_error!("ptr: *mut findToDo is null!");
        assert!(ptr != std::ptr::null_mut());
    }

    let controler = &mut *ptr;

    thread::spawn(move || {
        println!("insertToDo thread::spawn");
        controler.find_todo_by_id(toDoId);
    });
}

#[no_mangle]
pub unsafe extern "C" fn pj_fetch_todo_data(ptr: *mut PJToDoController) {
    if ptr == std::ptr::null_mut() {
        pj_error!("ptr : *mut fetchData is null!");
        assert!(ptr != std::ptr::null_mut());
    }

    let controler = &mut *ptr;

    thread::spawn(move || {
        println!("fetchToDoData thread::spawn");
        controler.fetch_data();
    });
}

#[no_mangle]
pub unsafe extern "C" fn pj_update_overdue_todos(ptr: *const PJToDoController) {
    if ptr == std::ptr::null_mut() {
        pj_error!("ptr: *mut updateToDo is null!");
        assert!(ptr != std::ptr::null_mut());
    }

    let controler = &*ptr;

    thread::spawn(move || {
        println!("updateOverDueToDos thread::spawn");
        controler.update_overdue_todos();
    });
}

#[no_mangle]
pub unsafe extern "C" fn pj_todo_title_at_section(
    ptr: *const PJToDoController,
    section: i32,
) -> *mut c_char {
    if ptr == std::ptr::null_mut() {
        pj_error!("ptr : *const todo_title_at_section is null!");
        assert!(ptr != std::ptr::null_mut());
    }
    let controler = &*ptr;
    controler.todo_title_at_section(section)
}

#[no_mangle]
pub unsafe extern "C" fn pj_todo_at_section(
    ptr: *const PJToDoController,
    section: i32,
    index: i32,
) -> *const ToDoQuery {
    if ptr == std::ptr::null_mut() {
        pj_error!("ptr or toDo: *mut todoAtIndex is null!");
        assert!(ptr != std::ptr::null_mut());
    }
    let controler = &*ptr;
    controler.todo_at_section(section, index)
}

#[no_mangle]
pub unsafe extern "C" fn pj_get_todo_number_of_sections(ptr: *const PJToDoController) -> i32 {
    if ptr == std::ptr::null_mut() {
        pj_error!("ptr or toDo: *mut getToDoCount is null!");
        assert!(ptr != std::ptr::null_mut());
    }
    let controler = &*ptr;
    controler.get_count_of_sections() as i32
}

#[no_mangle]
pub unsafe extern "C" fn pj_get_todo_counts_at_section(
    ptr: *const PJToDoController,
    section: i32,
) -> i32 {
    if ptr == std::ptr::null_mut() {
        pj_error!("ptr or toDo: *mut getToDoCount is null!");
        assert!(ptr != std::ptr::null_mut());
    }
    let controler = &*ptr;
    controler.get_count_at_section(section) as i32
}

#[no_mangle]
pub unsafe extern "C" fn pj_todo_type_with_id(
    ptr: *const PJToDoController,
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
pub unsafe extern "C" fn pj_todo_tag_with_id(
    ptr: *const PJToDoController,
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
pub unsafe extern "C" fn pj_free_rust_PJToDoController(ptr: *mut PJToDoController) {
    if ptr != std::ptr::null_mut() {
        Box::from_raw(ptr); //unsafe
    }
}
