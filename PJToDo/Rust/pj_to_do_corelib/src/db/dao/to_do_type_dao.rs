use to_do_type::to_do_type::ToDoTypeInsert;

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
}
