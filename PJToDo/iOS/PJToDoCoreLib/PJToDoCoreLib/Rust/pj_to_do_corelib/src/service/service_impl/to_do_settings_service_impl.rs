use db::dao::to_do_settings_dao::PJToDoSettingsDAO;
use db::dao::dao_impl::to_do_settings_dao_impl::{createPJToDoSettingsDAOImpl};
use service::to_do_settings_service::PJToDoSettingsService;
use mine::todo_settings::{ToDoSettingsInsert, ToDoSettings};

#[repr(C)]
pub struct PJToDoSettingsServiceImpl {
    pub todo_settings_dao: Box<dyn PJToDoSettingsDAO>,
}

impl PJToDoSettingsService for PJToDoSettingsServiceImpl {
    /**
     * 添加分类
     */
    fn insert_todo_settings(
        &self,
        to_do_settings: &ToDoSettingsInsert,
    ) -> Result<usize, diesel::result::Error> {
        self.todo_settings_dao.insert_todo_settings(to_do_settings)
    }

    fn delete_todo_settings(&self, to_do_settings_id: i32) -> Result<usize, diesel::result::Error> {
        self.todo_settings_dao
            .delete_todo_settings(to_do_settings_id)
    }

    fn update_todo_settings(
        &self,
        to_do_settings: &ToDoSettings,
    ) -> Result<usize, diesel::result::Error> {
        self.todo_settings_dao.update_todo_settings(to_do_settings)
    }

    fn fetch_data(&self) -> Result<Vec<ToDoSettings>, diesel::result::Error> {
        self.todo_settings_dao.fetch_data()
    }
}

impl Drop for PJToDoSettingsServiceImpl {
    fn drop(&mut self) {
        println!("PJToDoSettingsServiceImpl -> drop");
    }
}

/*** extern "C" ***/

pub fn createPJToDoSettingsServiceImpl() -> impl PJToDoSettingsService {
    let service = PJToDoSettingsServiceImpl {
        todo_settings_dao: Box::new(createPJToDoSettingsDAOImpl()),
    };
    service
}
