use to_do::to_do::{ToDoInsert, ToDoQuery};
use db::dao::to_do_dao::PJToDoDAO;
use db::pj_db_connection_util::pj_db_connection_util::StaticPJDBConnectionUtil;
use db::tables::schema;
use diesel::prelude::*;
#[allow(unused_imports)]
use common::pj_logger::PJLogger;
use db::tables::schema::todo::dsl::*;

pub struct PJToDoDAOImpl;

impl PJToDoDAO for PJToDoDAOImpl {
    fn insert_todo(&self, to_do: &ToDoInsert) -> Result<usize, diesel::result::Error> {
        pj_info!("insert_todo: to_do: {:?}", to_do);
        let inserted_result = diesel::insert_into(schema::todo::table)
            .values(to_do)
            .execute(&StaticPJDBConnectionUtil.connection);
        match inserted_result {
            Ok(inserted_row) => {
                let mut result: Result<usize, diesel::result::Error> =
                    Err(diesel::result::Error::NotFound);;
                if inserted_row == 1 {
                    result = Ok(inserted_row);
                } else {
                    pj_error!("insert_todo: insert failure!");
                }
                pj_info!("insert_todo: inserted_row: {}", inserted_row);
                result
            }
            Err(e) => {
                pj_error!("insert_todo: insert failure {}!", e);
                Err(e)
            }
        }
    }

    fn delete_todo(&self, to_do_id: i32) -> Result<usize, diesel::result::Error> {
        let deleted_result = diesel::delete(todo.filter(id.eq(to_do_id)))
            .execute(&StaticPJDBConnectionUtil.connection);
        match deleted_result {
            Ok(deleted_row) => {
                let mut result: Result<usize, diesel::result::Error> =
                    Err(diesel::result::Error::NotFound);
                if deleted_row == 1 {
                    result = Ok(deleted_row);
                } else {
                    pj_error!("delete_todo: delete failure!");
                }
                pj_info!("delete_todo: delete_row: {}", deleted_row);
                result
            }
            Err(e) => {
                pj_error!("delete_todo: delete failure {}", e);
                Err(e)
            }
        }
    }

    fn update_todo(&self, to_do: &ToDoQuery) -> Result<usize, diesel::result::Error> {
        pj_info!("insert_todo: to_do: {:?}", to_do);
        let update_result = diesel::update(todo.filter(id.eq(to_do.id)))
            .set(to_do)
            .execute(&StaticPJDBConnectionUtil.connection);

        match update_result {
            Ok(update_row) => {
                let mut result: Result<usize, diesel::result::Error> =
                    Err(diesel::result::Error::NotFound);
                if update_row == 1 {
                    result = Ok(update_row);
                } else {
                    pj_error!("update_todo: update failure!");
                }
                pj_info!("update_todo: update_row: {}", update_row);
                result
            }
            Err(e) => {
                pj_error!("update_todo: update failure {}", e);
                Err(e)
            }
        }
    }

    fn find_todo_by_id(&self, to_do_id: i32) -> Result<ToDoQuery, diesel::result::Error> {
        pj_info!("find_todo_by_id: to_do_id: {:?}", to_do_id);
        let find_result = todo
            .find(to_do_id)
            .first::<ToDoQuery>(&StaticPJDBConnectionUtil.connection);
        match find_result {
            Ok(result) => {
                pj_info!("find_todo_by_id success!");
                Ok(result)
            }
            Err(e) => {
                pj_error!("find_todo_by_id failure {}", e);
                Err(e)
            }
        }
    }

    fn find_todo_by_title(&self, todo_title: String) -> Result<ToDoQuery, diesel::result::Error> {
        pj_info!("find_todo_by_title: {}", todo_title);

        let to_dos_result = schema::todo::table
            .filter(title.eq(title))
            .load::<ToDoQuery>(&StaticPJDBConnectionUtil.connection);

        match to_dos_result {
            Ok(to_dos) => {
                pj_info!("find_todo_by_title success!");
                let first_todo = to_dos.first();
                match first_todo {
                    Some(to_do_query) => Ok(ToDoQuery::from(to_do_query)),
                    None => Err(diesel::result::Error::NotFound),
                }
            }
            Err(e) => {
                pj_error!("find_todo_by_title failure {}", e);
                Err(e)
            }
        }
    }

    fn find_todo_like_title(
        &self,
        todo_title: String,
    ) -> Result<Vec<ToDoQuery>, diesel::result::Error> {
        let like = format!("%{}%", todo_title);
        let like_result = schema::todo::table
            .filter(title.like(&like))
            .order(id.asc())
            .load(&StaticPJDBConnectionUtil.connection);
        match like_result {
            Ok(to_dos) => {
                pj_info!("find_todo_like_title success!");
                Ok(to_dos)
            }
            Err(e) => {
                pj_error!("find_todo_like_title failure {}", e);
                Err(e)
            }
        }
    }

    fn fetch_data(&self) -> Result<Vec<ToDoQuery>, diesel::result::Error> {
        let to_dos_result = todo.load::<ToDoQuery>(&StaticPJDBConnectionUtil.connection);
        match to_dos_result {
            Ok(to_dos) => {
                pj_info!("fetchData success!");
                Ok(to_dos)
            }
            Err(e) => {
                pj_error!("fetchData failure {}", e);
                Err(e)
            }
        }
    }
}

pub fn createPJToDoDAOImpl() -> impl PJToDoDAO {
    PJToDoDAOImpl {}
}
