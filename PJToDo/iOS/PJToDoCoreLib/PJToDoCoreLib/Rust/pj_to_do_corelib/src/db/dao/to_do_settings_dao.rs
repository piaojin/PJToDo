use crate::mine::todo_settings::{ToDoSettingsInsert, ToDoSettings};

pub trait PJToDoSettingsDAO {
    // add code here
    /**
     * 添加分类
     */
    fn insert_todo_settings(
        &self,
        to_do_settings: &ToDoSettingsInsert,
    ) -> Result<usize, diesel::result::Error>;

    fn delete_todo_settings(&self, to_do_settings_id: i32) -> Result<usize, diesel::result::Error>;

    fn update_todo_settings(
        &self,
        to_do_settings: &ToDoSettings,
    ) -> Result<usize, diesel::result::Error>;

    fn fetch_data(&self) -> Result<Vec<ToDoSettings>, diesel::result::Error>;
}
