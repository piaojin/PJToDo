use to_do_type::to_do_type::ToDoTypeInsert;

pub trait PJToDoTypeService {
    /**
     * add
     */
    fn insert_to_do_type(
        &self,
        to_do_type: &ToDoTypeInsert,
    ) -> Result<usize, diesel::result::Error>;

    fn delete_to_do_type(&self, to_do_type_id: i32) -> Result<usize, diesel::result::Error>;
}

pub fn insert_to_do_type(
    todo_type_service: &Box<dyn PJToDoTypeService>,
    to_do_type: &ToDoTypeInsert,
) -> Result<usize, diesel::result::Error> {
    todo_type_service.insert_to_do_type(to_do_type)
}

pub fn delete_to_do_type(
    todo_type_service: &Box<dyn PJToDoTypeService>,
    to_do_type_id: i32,
) -> Result<usize, diesel::result::Error> {
    todo_type_service.delete_to_do_type(to_do_type_id)
}
