use db::dao::to_do_type_dao::PJToDoTypeDAO;
use db::dao::dao_impl::to_do_type_dao_impl::{createPJToDoTypeDAOImpl, PJToDoTypeDAOImpl};
use service::to_do_type_service::PJToDoTypeService;
use to_do_type::to_do_type::ToDoTypeInsert;

#[repr(C)]
pub struct PJToDoTypeServiceImpl {
    pub todo_type_dao: Box<dyn PJToDoTypeDAO>,
}

impl PJToDoTypeService for PJToDoTypeServiceImpl {
    /**
     * 添加分类
     */
    fn insert_to_do_type(&self, to_do_type: &ToDoTypeInsert) -> Result<usize, String> {
        self.todo_type_dao.insert_to_do_type(to_do_type)
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
