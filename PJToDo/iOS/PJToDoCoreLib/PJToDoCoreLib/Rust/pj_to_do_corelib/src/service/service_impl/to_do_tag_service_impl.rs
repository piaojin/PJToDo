use crate::db::dao::to_do_tag_dao::PJToDoTagDAO;
use crate::db::dao::dao_impl::to_do_tag_dao_impl::{createPJToDoTagDAOImpl};
use crate::service::to_do_tag_service::PJToDoTagService;
use crate::to_do_tag::to_do_tag::{ToDoTagInsert, ToDoTag};

#[repr(C)]
pub struct PJToDoTagServiceImpl {
    pub todo_tag_dao: Box<dyn PJToDoTagDAO>,
}

impl PJToDoTagService for PJToDoTagServiceImpl {
    /**
     * 添加分类
     */
    fn insert_todo_tag(&self, to_do_tag: &ToDoTagInsert) -> Result<usize, diesel::result::Error> {
        self.todo_tag_dao.insert_todo_tag(to_do_tag)
    }

    fn delete_todo_tag(&self, to_do_tag_id: i32) -> Result<usize, diesel::result::Error> {
        self.todo_tag_dao.delete_todo_tag(to_do_tag_id)
    }

    fn update_todo_tag(&self, to_do_tag: &ToDoTag) -> Result<usize, diesel::result::Error> {
        self.todo_tag_dao.update_todo_tag(to_do_tag)
    }

    fn find_todo_tag_by_id(&self, to_do_tag_id: i32) -> Result<ToDoTag, diesel::result::Error> {
        self.todo_tag_dao.find_todo_tag_by_id(to_do_tag_id)
    }

    fn find_todo_tag_by_name(&self, name: String) -> Result<ToDoTag, diesel::result::Error> {
        self.todo_tag_dao.find_todo_tag_by_name(name)
    }

    fn fetch_data(&self) -> Result<Vec<ToDoTag>, diesel::result::Error> {
        self.todo_tag_dao.fetch_data()
    }
}

impl Drop for PJToDoTagServiceImpl {
    fn drop(&mut self) {
        println!("PJToDoTagServiceImpl -> drop");
    }
}

/*** extern "C" ***/

pub fn createPJToDoTagServiceImpl() -> impl PJToDoTagService {
    let service = PJToDoTagServiceImpl {
        todo_tag_dao: Box::new(createPJToDoTagDAOImpl()),
    };
    service
}
