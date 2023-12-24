use crate::to_do_tag::to_do_tag::{ToDoTagInsert, ToDoTag};

pub trait PJToDoTagDAO {
    fn insert_todo_tag(&self, to_do_tag: &ToDoTagInsert) -> Result<usize, diesel::result::Error>;

    fn delete_todo_tag(&self, to_do_tag_id: i32) -> Result<usize, diesel::result::Error>;

    fn update_todo_tag(&self, to_do_tag: &ToDoTag) -> Result<usize, diesel::result::Error>;

    fn find_todo_tag_by_id(&self, to_do_tag_id: i32) -> Result<ToDoTag, diesel::result::Error>;

    fn find_todo_tag_by_name(&self, name: String) -> Result<ToDoTag, diesel::result::Error>;

    fn fetch_data(&self) -> Result<Vec<ToDoTag>, diesel::result::Error>;
}
