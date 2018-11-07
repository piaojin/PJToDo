use to_do::to_do::{ToDoInsert, ToDoQuery};

pub trait PJToDoService {
    fn insert_todo(&self, to_do: &ToDoInsert) -> Result<usize, diesel::result::Error>;

    fn delete_todo(&self, to_do_id: i32) -> Result<usize, diesel::result::Error>;

    fn update_todo(&self, to_do: &ToDoQuery) -> Result<usize, diesel::result::Error>;

    fn find_todo_by_id(&self, to_do_id: i32) -> Result<ToDoQuery, diesel::result::Error>;

    fn find_todo_by_title(&self, todo_title: String) -> Result<ToDoQuery, diesel::result::Error>;

    fn fetch_data(&self) -> Result<Vec<ToDoQuery>, diesel::result::Error>;

    fn find_todo_like_title(
        &self,
        todo_title: String,
    ) -> Result<Vec<ToDoQuery>, diesel::result::Error>;
}

pub fn insert_todo(
    todo_service: &Box<dyn PJToDoService>,
    to_do: &ToDoInsert,
) -> Result<usize, diesel::result::Error> {
    todo_service.insert_todo(to_do)
}

pub fn delete_todo(
    todo_service: &Box<dyn PJToDoService>,
    to_do_id: i32,
) -> Result<usize, diesel::result::Error> {
    todo_service.delete_todo(to_do_id)
}

pub fn update_todo(
    todo_service: &Box<dyn PJToDoService>,
    to_do: &ToDoQuery,
) -> Result<usize, diesel::result::Error> {
    todo_service.update_todo(to_do)
}

pub fn find_todo_by_id(
    todo_service: &Box<dyn PJToDoService>,
    to_do_id: i32,
) -> Result<ToDoQuery, diesel::result::Error> {
    todo_service.find_todo_by_id(to_do_id)
}

pub fn find_todo_by_title(
    todo_service: &Box<dyn PJToDoService>,
    title: String,
) -> Result<ToDoQuery, diesel::result::Error> {
    todo_service.find_todo_by_title(title)
}

pub fn fetch_data(
    todo_service: &Box<dyn PJToDoService>,
) -> Result<Vec<ToDoQuery>, diesel::result::Error> {
    todo_service.fetch_data()
}

pub fn find_todo_like_title(
    todo_service: &Box<dyn PJToDoService>,
    title: String,
) -> Result<Vec<ToDoQuery>, diesel::result::Error> {
    todo_service.find_todo_like_title(title)
}
