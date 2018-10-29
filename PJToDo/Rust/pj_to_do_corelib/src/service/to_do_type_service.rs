use to_do_type::to_do_type::ToDoTypeInsert;

pub trait PJToDoTypeService {
    // add code here
    /**
     * 添加分类
     */
    fn insert_to_do_type(&self, to_do_type: &ToDoTypeInsert) -> Result<usize, String>;
}

pub fn insert_to_do_type(
    todo_type_service: &Box<dyn PJToDoTypeService>,
    to_do_type: &ToDoTypeInsert,
) -> Result<usize, String> {
    todo_type_service.insert_to_do_type(to_do_type)
}
