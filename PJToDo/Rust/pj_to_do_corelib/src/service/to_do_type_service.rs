use to_do_type::to_do_type::{ToDoTypeInsert, ToDoType};

pub trait PJToDoTypeService {
    /**
     * add
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

pub fn update_to_do_type(
    todo_type_service: &Box<dyn PJToDoTypeService>,
    to_do_type: &ToDoType,
) -> Result<usize, diesel::result::Error> {
    todo_type_service.update_to_do_type(to_do_type)
}

pub fn find_to_do_type_by_id(
    todo_type_service: &Box<dyn PJToDoTypeService>,
    to_do_type_id: i32,
) -> Result<ToDoType, diesel::result::Error> {
    todo_type_service.find_to_do_type_by_id(to_do_type_id)
}

pub fn find_to_do_type_by_name(
    todo_type_service: &Box<dyn PJToDoTypeService>,
    type_name: String,
) -> Result<ToDoType, diesel::result::Error> {
    todo_type_service.find_to_do_type_by_name(type_name)
}
