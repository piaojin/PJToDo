use delegates::to_do_settings_delegate::{IPJToDoSettingsDelegate, IPJToDoSettingsDelegateWrapper};
use service::to_do_settings_service::{PJToDoSettingsService, insert_todo_settings, delete_todo_settings, update_todo_settings, fetch_data};
use service::service_impl::to_do_settings_service_impl::{createPJToDoSettingsServiceImpl};
use mine::todo_settings::{ToDoSettings, ToDoSettingsInsert, createToDoSettingsInsert};
use common::{free_rust_any_object};
#[allow(unused_imports)]
use common::pj_logger::PJLogger;
use std::thread;
use std::marker::{Send, Sync};

/*
* cbindgen didn't support Box<dyn PJToDoSettingsService> type,so I need to use PJToDoSettingsServiceController to define Box<dyn PJToDoSettingsService>.
*/
#[repr(C)]
pub struct PJToDoSettingsServiceController {
    todo_settings_service: Box<dyn PJToDoSettingsService>,
}

impl PJToDoSettingsServiceController {
    fn new() -> PJToDoSettingsServiceController {
        let serviceController = PJToDoSettingsServiceController {
            todo_settings_service: Box::new(createPJToDoSettingsServiceImpl()),
        };
        serviceController
    }
}

impl Drop for PJToDoSettingsServiceController {
    fn drop(&mut self) {
        println!("PJToDoSettingsServiceController -> drop");
    }
}

unsafe impl Send for PJToDoSettingsServiceController {}
unsafe impl Sync for PJToDoSettingsServiceController {}

/*The create and free are both in Rust. only free PJToDoSettingsController in Swift.*/
#[repr(C)]
pub struct PJToDoSettingsController {
    pub delegate: IPJToDoSettingsDelegate,
    pub todo_settings_service_controller: *mut PJToDoSettingsServiceController,
    pub insert_todo_settings: *mut ToDoSettingsInsert,
    pub todo_settings: *mut Vec<ToDoSettings>,
}

unsafe impl Send for PJToDoSettingsController {}
unsafe impl Sync for PJToDoSettingsController {}

impl PJToDoSettingsController {
    fn new(delegate: IPJToDoSettingsDelegate) -> PJToDoSettingsController {
        let controller = PJToDoSettingsController {
            delegate: delegate,
            todo_settings_service_controller: Box::into_raw(Box::new(
                PJToDoSettingsServiceController::new(),
            )),
            insert_todo_settings: std::ptr::null_mut(),
            todo_settings: std::ptr::null_mut(),
        };
        controller
    }

    /**
     * 添加分类
     */
    pub unsafe fn insert_todo_settings(&mut self, to_do_settings: *mut ToDoSettingsInsert) {
        pj_info!("insert_todo_settings: {}", (*to_do_settings).remind_email);
        assert!(!to_do_settings.is_null());
        let i_delegate =
            IPJToDoSettingsDelegateWrapper((&self.delegate) as *const IPJToDoSettingsDelegate);

        let result = insert_todo_settings(
            &(&(*self.todo_settings_service_controller)).todo_settings_service,
            &(*to_do_settings),
        );

        /*free the old todoSettingsInsert before set the new one*/
        free_rust_any_object(self.insert_todo_settings);
        self.insert_todo_settings = to_do_settings;

        match result {
            Ok(_) => {
                (i_delegate.insert_result)(i_delegate.user, true);
            }
            Err(_e) => {
                (i_delegate.insert_result)(i_delegate.user, false);
            }
        }
    }

    pub unsafe fn delete_todo_settings(&self, to_do_settings_id: i32) {
        let i_delegate =
            IPJToDoSettingsDelegateWrapper((&self.delegate) as *const IPJToDoSettingsDelegate);

        let result = delete_todo_settings(
            &(&(*self.todo_settings_service_controller)).todo_settings_service,
            to_do_settings_id,
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

    pub unsafe fn update_todo_settings(&self, to_do_settings: *const ToDoSettings) {
        assert!(!to_do_settings.is_null());

        let i_delegate =
            IPJToDoSettingsDelegateWrapper((&self.delegate) as *const IPJToDoSettingsDelegate);

        let result = update_todo_settings(
            &(&(*self.todo_settings_service_controller)).todo_settings_service,
            &(*to_do_settings),
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

    pub unsafe fn fetch_data(&mut self) {
        let i_delegate =
            IPJToDoSettingsDelegateWrapper((&self.delegate) as *const IPJToDoSettingsDelegate);

        let result = fetch_data(&(&(*self.todo_settings_service_controller)).todo_settings_service);

        match result {
            Ok(to_do_settingss) => {
                /*free the old todo_settings before set the new one*/
                free_rust_any_object(self.todo_settings);
                self.todo_settings = Box::into_raw(Box::new(to_do_settingss));
                (i_delegate.fetch_data_result)(i_delegate.user, true);
            }
            Err(_e) => {
                (i_delegate.fetch_data_result)(i_delegate.user, false);
            }
        }
    }

    pub unsafe fn todo_settings_at_index(&self, index: i32) -> *const ToDoSettings {
        let index: usize = index as usize;
        assert!(index <= self.get_count());

        let mut todo_settings: *const ToDoSettings = std::ptr::null_mut();
        if self.get_count() > 0 {
            todo_settings = &((*(self.todo_settings))[index]);
        }
        todo_settings
    }

    pub unsafe fn get_count(&self) -> usize {
        if self.todo_settings.is_null() {
            ()
        }

        let count = (*(self.todo_settings)).len();
        count
    }
}

impl Drop for PJToDoSettingsController {
    fn drop(&mut self) {
        //PJToDoSettingsController被释放，告诉当前持有PJToDoSettingsDelegate对象的所有权者做相应的处理
        unsafe {
            free_rust_any_object(self.todo_settings_service_controller);
            free_rust_any_object(self.insert_todo_settings);
            free_rust_any_object(self.todo_settings);
        }
        println!("PJToDoSettingsController -> drop");
    }
}

/*** extern "C" ***/

#[no_mangle]
pub extern "C" fn createPJToDoSettingsController(
    delegate: IPJToDoSettingsDelegate,
) -> *mut PJToDoSettingsController {
    let controller = PJToDoSettingsController::new(delegate);
    Box::into_raw(Box::new(controller))
}

#[no_mangle]
pub unsafe extern "C" fn insertToDoSettings(
    ptr: *mut PJToDoSettingsController,
    toDoSettings: *mut ToDoSettingsInsert,
) {
    if ptr.is_null() || toDoSettings.is_null() {
        pj_error!("ptr or toDoSettings: *mut insertToDoSettings is null!");
        assert!(!ptr.is_null() && !toDoSettings.is_null());
    }

    let controler = &mut *ptr;
    let toDoSettings = &mut *toDoSettings;

    thread::spawn(move || {
        println!("insertToDoSettings thread::spawn");
        controler.insert_todo_settings(toDoSettings);
    });
}

#[no_mangle]
pub unsafe extern "C" fn deleteToDoSettings(
    ptr: *mut PJToDoSettingsController,
    toDoSettingsId: i32,
) {
    if ptr.is_null() {
        pj_error!("ptr: *mut deleteToDoSettings is null!");
        assert!(!ptr.is_null());
    }

    let controler = &mut *ptr;

    thread::spawn(move || {
        println!("insertToDoSettings thread::spawn");
        controler.delete_todo_settings(toDoSettingsId);
    });
}

#[no_mangle]
pub unsafe extern "C" fn updateToDoSettings(
    ptr: *mut PJToDoSettingsController,
    toDoSettings: *const ToDoSettings,
) {
    if ptr.is_null() || toDoSettings.is_null() {
        pj_error!("ptr or toDoSettings: *mut updateToDoSettings is null!");
        assert!(!ptr.is_null() && !toDoSettings.is_null());
    }

    let controler = &mut *ptr;
    let toDoSettings = &*toDoSettings;

    thread::spawn(move || {
        println!("insertToDoSettings thread::spawn");
        controler.update_todo_settings(toDoSettings);
    });
}

#[no_mangle]
pub unsafe extern "C" fn fetchToDoSettingsData(ptr: *mut PJToDoSettingsController) {
    if ptr.is_null() {
        pj_error!("ptr or toDoSettings: *mut fetchData is null!");
        assert!(!ptr.is_null());
    }
    let controler = &mut *ptr;
    controler.fetch_data()
}

#[no_mangle]
pub unsafe extern "C" fn todoSettingsAtIndex(
    ptr: *const PJToDoSettingsController,
    index: i32,
) -> *const ToDoSettings {
    if ptr.is_null() {
        pj_error!("ptr or toDoSettings: *mut todoSettingsAtIndex is null!");
        assert!(!ptr.is_null());
    }
    let controler = &*ptr;
    controler.todo_settings_at_index(index)
}

#[no_mangle]
pub unsafe extern "C" fn getToDoSettingsCount(ptr: *const PJToDoSettingsController) -> i32 {
    if ptr.is_null() {
        pj_error!("ptr or toDoSettings: *mut getToDoSettingsCount is null!");
        assert!(!ptr.is_null());
    }
    let controler = &*ptr;
    controler.get_count() as i32
}

#[no_mangle]
pub unsafe extern "C" fn free_rust_PJToDoSettingsController(ptr: *mut PJToDoSettingsController) {
    if !ptr.is_null() {
        Box::from_raw(ptr); //unsafe
    };
}
