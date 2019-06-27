use mine::todo_settings::{ToDoSettingsInsert, ToDoSettings};

pub trait PJToDoSettingsService {
    /**
     * add
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

pub fn insert_todo_settings(
    todo_settings_service: &Box<dyn PJToDoSettingsService>,
    to_do_settings: &ToDoSettingsInsert,
) -> Result<usize, diesel::result::Error> {
    todo_settings_service.insert_todo_settings(to_do_settings)
}

pub fn delete_todo_settings(
    todo_settings_service: &Box<dyn PJToDoSettingsService>,
    to_do_settings_id: i32,
) -> Result<usize, diesel::result::Error> {
    todo_settings_service.delete_todo_settings(to_do_settings_id)
}

pub fn update_todo_settings(
    todo_settings_service: &Box<dyn PJToDoSettingsService>,
    to_do_settings: &ToDoSettings,
) -> Result<usize, diesel::result::Error> {
    todo_settings_service.update_todo_settings(to_do_settings)
}

pub fn fetch_data(
    todo_settings_service: &Box<dyn PJToDoSettingsService>,
) -> Result<Vec<ToDoSettings>, diesel::result::Error> {
    todo_settings_service.fetch_data()
}
