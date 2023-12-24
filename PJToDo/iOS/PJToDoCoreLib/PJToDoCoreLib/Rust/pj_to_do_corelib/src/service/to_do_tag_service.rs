use crate::to_do_tag::to_do_tag::{ToDoTagInsert, ToDoTag};

pub trait PJToDoTagService {
    /**
     * add
     */
    fn insert_todo_tag(&self, to_do_tag: &ToDoTagInsert) -> Result<usize, diesel::result::Error>;

    fn delete_todo_tag(&self, to_do_tag_id: i32) -> Result<usize, diesel::result::Error>;

    fn update_todo_tag(&self, to_do_tag: &ToDoTag) -> Result<usize, diesel::result::Error>;

    fn find_todo_tag_by_id(&self, to_do_tag_id: i32) -> Result<ToDoTag, diesel::result::Error>;

    fn find_todo_tag_by_name(&self, name: String) -> Result<ToDoTag, diesel::result::Error>;

    fn fetch_data(&self) -> Result<Vec<ToDoTag>, diesel::result::Error>;
}

pub fn insert_todo_tag(
    todo_tag_service: &Box<dyn PJToDoTagService>,
    to_do_tag: &ToDoTagInsert,
) -> Result<usize, diesel::result::Error> {
    todo_tag_service.insert_todo_tag(to_do_tag)
}

pub fn delete_todo_tag(
    todo_tag_service: &Box<dyn PJToDoTagService>,
    to_do_tag_id: i32,
) -> Result<usize, diesel::result::Error> {
    todo_tag_service.delete_todo_tag(to_do_tag_id)
}

pub fn update_todo_tag(
    todo_tag_service: &Box<dyn PJToDoTagService>,
    to_do_tag: &ToDoTag,
) -> Result<usize, diesel::result::Error> {
    todo_tag_service.update_todo_tag(to_do_tag)
}

pub fn find_todo_tag_by_id(
    todo_tag_service: &Box<dyn PJToDoTagService>,
    to_do_tag_id: i32,
) -> Result<ToDoTag, diesel::result::Error> {
    todo_tag_service.find_todo_tag_by_id(to_do_tag_id)
}

pub fn find_todo_tag_by_name(
    todo_tag_service: &Box<dyn PJToDoTagService>,
    tag_name: String,
) -> Result<ToDoTag, diesel::result::Error> {
    todo_tag_service.find_todo_tag_by_name(tag_name)
}

pub fn fetch_data(
    todo_tag_service: &Box<dyn PJToDoTagService>,
) -> Result<Vec<ToDoTag>, diesel::result::Error> {
    todo_tag_service.fetch_data()
}
