use to_do_type::to_do_type::ToDoTypeInsert;

pub trait PJToDoTypeDAO {
    // add code here
    /**
     * 添加分类
     */
    fn insert_to_do_type(&self, to_do_type: ToDoTypeInsert) -> Result<usize, String>;
}
