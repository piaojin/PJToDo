use to_do_type::to_do_type::{ToDoTypeInsert, ToDoType};

pub trait PJToDoTypeDAO {
    // add code here
    /**
     * 添加分类
     */
    fn insert_todo_type(&self, to_do_type: &ToDoTypeInsert)
        -> Result<usize, diesel::result::Error>;

    fn delete_todo_type(&self, to_do_type_id: i32) -> Result<usize, diesel::result::Error>;

    fn update_todo_type(&self, to_do_type: &ToDoType) -> Result<usize, diesel::result::Error>;

    fn find_todo_type_by_id(&self, to_do_type_id: i32) -> Result<ToDoType, diesel::result::Error>;

    fn find_todo_type_by_name(&self, name: String) -> Result<ToDoType, diesel::result::Error>;

    fn fetch_data(&self) -> Result<Vec<ToDoType>, diesel::result::Error>;
}
