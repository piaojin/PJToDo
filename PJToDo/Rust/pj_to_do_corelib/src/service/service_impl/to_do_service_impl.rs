use db::dao::to_do_dao::PJToDoDAO;
use db::dao::dao_impl::to_do_dao_impl::{createPJToDoDAOImpl};
use service::to_do_service::PJToDoService;
use to_do::to_do::{ToDoInsert, ToDoQuery};

#[repr(C)]
pub struct PJToDoServiceImpl {
    pub todo_dao: Box<dyn PJToDoDAO>,
}

impl PJToDoService for PJToDoServiceImpl {
    /**
     * 添加分类
     */
    fn insert_todo(&self, to_do: &ToDoInsert) -> Result<usize, diesel::result::Error> {
        self.todo_dao.insert_todo(to_do)
    }

    fn delete_todo(&self, to_do_id: i32) -> Result<usize, diesel::result::Error> {
        self.todo_dao.delete_todo(to_do_id)
    }

    fn update_todo(&self, to_do: &ToDoQuery) -> Result<usize, diesel::result::Error> {
        self.todo_dao.update_todo(to_do)
    }

    fn find_todo_by_id(&self, to_do_id: i32) -> Result<ToDoQuery, diesel::result::Error> {
        self.todo_dao.find_todo_by_id(to_do_id)
    }

    fn find_todo_by_title(&self, todo_title: String) -> Result<ToDoQuery, diesel::result::Error> {
        self.todo_dao.find_todo_by_title(todo_title)
    }

    fn fetch_data(&self) -> Result<Vec<ToDoQuery>, diesel::result::Error> {
        self.todo_dao.fetch_data()
    }

    fn find_todo_like_title(
        &self,
        todo_title: String,
    ) -> Result<Vec<ToDoQuery>, diesel::result::Error> {
        self.todo_dao.find_todo_like_title(todo_title)
    }

    fn find_todo_date_future_day_more_than(
        &self,
        from_day: String,
        comparison_days: i32,
    ) -> Result<Vec<ToDoQuery>, diesel::result::Error> {
        self.todo_dao
            .find_todo_date_future_day_more_than(from_day, comparison_days)
    }

    fn fetch_todos_order_by_state(&self) -> Result<Vec<Vec<ToDoQuery>>, diesel::result::Error> {
        self.todo_dao.fetch_todos_order_by_state()
    }

    fn update_overdue_todos(&self) -> Result<Vec<ToDoQuery>, diesel::result::Error> {
        self.todo_dao.update_overdue_todos()
    }
}

impl Drop for PJToDoServiceImpl {
    fn drop(&mut self) {
        println!("PJToDoServiceImpl -> drop");
    }
}

/*** extern "C" ***/

pub fn createPJToDoServiceImpl() -> impl PJToDoService {
    let service = PJToDoServiceImpl {
        todo_dao: Box::new(createPJToDoDAOImpl()),
    };
    service
}
