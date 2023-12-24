use crate::to_do::to_do::{ToDoInsert, ToDoQuery};

pub trait PJToDoDAO {
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

    fn find_todo_date_future_day_more_than(
        &self,
        from_day: String,
        comparison_days: i32,
    ) -> Result<Vec<ToDoQuery>, diesel::result::Error>;

    fn fetch_todos_order_by_state(&self) -> Result<Vec<Vec<ToDoQuery>>, diesel::result::Error>;

    fn update_overdue_todos(&self) -> Result<Vec<ToDoQuery>, diesel::result::Error>;
}
