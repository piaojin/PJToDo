use db::dao::to_do_type_dao::PJToDoTypeDAO;
use db::dao::dao_impl::to_do_type_dao_impl::{createPJToDoTypeDAOImpl, PJToDoTypeDAOImpl};
use service::to_do_type_service::PJToDoTypeService;
use to_do_type::to_do_type::{ToDoTypeInsert, ToDoType};

#[repr(C)]
pub struct PJToDoTypeServiceImpl {
    pub todo_type_dao: Box<dyn PJToDoTypeDAO>,
}

impl PJToDoTypeService for PJToDoTypeServiceImpl {
    /**
     * 添加分类
     */
    fn insert_to_do_type(
        &self,
        to_do_type: &ToDoTypeInsert,
    ) -> Result<usize, diesel::result::Error> {
        self.todo_type_dao.insert_to_do_type(to_do_type)
    }

    fn delete_to_do_type(&self, to_do_type_id: i32) -> Result<usize, diesel::result::Error> {
        self.todo_type_dao.delete_to_do_type(to_do_type_id)
    }

    fn update_to_do_type(&self, to_do_type: &ToDoType) -> Result<usize, diesel::result::Error> {
        self.todo_type_dao.update_to_do_type(to_do_type)
    }

    fn find_to_do_type_by_id(&self, to_do_type_id: i32) -> Result<ToDoType, diesel::result::Error> {
        self.todo_type_dao.find_to_do_type_by_id(to_do_type_id)
    }

    fn find_to_do_type_by_name(&self, name: String) -> Result<ToDoType, diesel::result::Error> {
        self.todo_type_dao.find_to_do_type_by_name(name)
    }
}

impl Drop for PJToDoTypeServiceImpl {
    fn drop(&mut self) {
        println!("PJToDoTypeServiceImpl -> drop");
    }
}

/*** extern "C" ***/

#[allow(non_snake_case)]
pub fn createPJToDoTypeServiceImpl() -> impl PJToDoTypeService {
    let service = PJToDoTypeServiceImpl {
        todo_type_dao: Box::new(createPJToDoTypeDAOImpl()),
    };
    service
}
