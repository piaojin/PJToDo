use to_do_type::to_do_type::{ToDoTypeInsert, ToDoType};

pub trait PJToDoTypeDAO {
    // add code here
    /**
     * 添加分类
     */
    fn insert_to_do_type(
        &self,
        to_do_type: &ToDoTypeInsert,
    ) -> Result<usize, diesel::result::Error>;

    fn delete_to_do_type(&self, to_do_type_id: i32) -> Result<usize, diesel::result::Error>;

    fn update_to_do_type(&self, to_do_type: &ToDoType) -> Result<usize, diesel::result::Error>;

    fn find_to_do_type_by_id(&self, to_do_type_id: i32) -> Result<ToDoType, diesel::result::Error>;

    fn find_to_do_type_by_name(&self, name: String) -> Result<ToDoType, diesel::result::Error>;
}
