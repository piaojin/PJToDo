use db::dao::to_do_type_dao::PJToDoTypeDAO;
use db::dao::dao_impl::to_do_type_dao_impl::{createPJToDoTypeDAOImpl};
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
    fn insert_todo_type(
        &self,
        to_do_type: &ToDoTypeInsert,
    ) -> Result<usize, diesel::result::Error> {
        self.todo_type_dao.insert_todo_type(to_do_type)
    }

    fn delete_todo_type(&self, to_do_type_id: i32) -> Result<usize, diesel::result::Error> {
        self.todo_type_dao.delete_todo_type(to_do_type_id)
    }

    fn update_todo_type(&self, to_do_type: &ToDoType) -> Result<usize, diesel::result::Error> {
        self.todo_type_dao.update_todo_type(to_do_type)
    }

    fn find_todo_type_by_id(&self, to_do_type_id: i32) -> Result<ToDoType, diesel::result::Error> {
        self.todo_type_dao.find_todo_type_by_id(to_do_type_id)
    }

    fn find_todo_type_by_name(&self, name: String) -> Result<ToDoType, diesel::result::Error> {
        self.todo_type_dao.find_todo_type_by_name(name)
    }

    fn fetch_data(&self) -> Result<Vec<ToDoType>, diesel::result::Error> {
        self.todo_type_dao.fetch_data()
    }
}

impl Drop for PJToDoTypeServiceImpl {
    fn drop(&mut self) {
        println!("PJToDoTypeServiceImpl -> drop");
    }
}

/*** extern "C" ***/

pub fn createPJToDoTypeServiceImpl() -> impl PJToDoTypeService {
    let service = PJToDoTypeServiceImpl {
        todo_type_dao: Box::new(createPJToDoTypeDAOImpl()),
    };
    service
}
