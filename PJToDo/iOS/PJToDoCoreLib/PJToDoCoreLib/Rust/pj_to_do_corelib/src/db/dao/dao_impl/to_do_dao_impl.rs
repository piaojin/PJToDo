use crate::db;
use crate::to_do::to_do::{ToDoInsert, ToDoQuery};
use db::dao::to_do_dao::PJToDoDAO;
use db::pj_db_connection_util::pj_db_connection_util::StaticPJDBConnectionUtil;
use db::tables::schema;
use diesel::prelude::*;
use db::tables::schema::todo::dsl::*;
use diesel::dsl::sql_query;

pub struct PJToDoDAOImpl;

impl PJToDoDAO for PJToDoDAOImpl {
    fn insert_todo(&self, to_do: &ToDoInsert) -> Result<usize, diesel::result::Error> {
        pj_info!("insert_todo: to_do: {:?}", to_do);
        let inserted_result = diesel::insert_into(schema::todo::table)
            .values(to_do)
            .execute(&(StaticPJDBConnectionUtil.lock().unwrap()).connection);
        match inserted_result {
            Ok(inserted_row) => {
                let mut result: Result<usize, diesel::result::Error> =
                    Err(diesel::result::Error::NotFound);
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
            .execute(&(StaticPJDBConnectionUtil.lock().unwrap()).connection);
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
            .execute(&(StaticPJDBConnectionUtil.lock().unwrap()).connection);

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
            .first::<ToDoQuery>(&(StaticPJDBConnectionUtil.lock().unwrap()).connection);
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
            .filter(title.eq(todo_title))
            .load::<ToDoQuery>(&(StaticPJDBConnectionUtil.lock().unwrap()).connection);

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
            .load(&(StaticPJDBConnectionUtil.lock().unwrap()).connection);
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
        let to_dos_result =
            todo.load::<ToDoQuery>(&(StaticPJDBConnectionUtil.lock().unwrap()).connection);
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

    fn find_todo_date_future_day_more_than(
        &self,
        from_day: String,
        comparison_days: i32,
    ) -> Result<Vec<ToDoQuery>, diesel::result::Error> {
        let sql = format!(
            "SELECT * FROM todo WHERE due_time < date('{}','{} days') and state == 0",
            from_day, comparison_days
        );
        let result = sql_query(sql)
            .load::<ToDoQuery>(&(StaticPJDBConnectionUtil.lock().unwrap()).connection);
        match result {
            Ok(to_dos) => {
                pj_info!("find_todo_date_future_day_more_than success!");
                Ok(to_dos)
            }
            Err(e) => {
                pj_error!("find_todo_date_future_day_more_than failure {}", e);
                Err(e)
            }
        }
    }

    fn fetch_todos_order_by_state(&self) -> Result<Vec<Vec<ToDoQuery>>, diesel::result::Error> {
        pj_info!("fetch_todos_order_by_state");

        let mut todos: Vec<Vec<ToDoQuery>> = Vec::new();
        let mut error: diesel::result::Error = diesel::result::Error::NotFound;
        let mut hasError = false;

        let todos_determined_result = schema::todo::table
            .filter(state.eq(0))
            .load::<ToDoQuery>(&(StaticPJDBConnectionUtil.lock().unwrap()).connection);

        match todos_determined_result {
            Ok(todos_determined) => {
                pj_info!("fetch_todos_order_by_state in type determined success!");
                todos.push(todos_determined);
            }
            Err(e) => {
                pj_error!(
                    "fetch_todos_order_by_state in type determined failure {}",
                    e
                );
                hasError = true;
                error = e;
            }
        }

        let todos_inprogress_result = schema::todo::table
            .filter(state.eq(1))
            .load::<ToDoQuery>(&(StaticPJDBConnectionUtil.lock().unwrap()).connection);

        match todos_inprogress_result {
            Ok(todos_inprogress) => {
                pj_info!("fetch_todos_order_by_state in type inprogress success!");
                todos.push(todos_inprogress);
            }
            Err(e) => {
                pj_error!(
                    "fetch_todos_order_by_state in type inprogress failure {}",
                    e
                );
                hasError = true;
                error = e;
            }
        }

        let todos_completed_result = schema::todo::table
            .filter(state.eq(2))
            .load::<ToDoQuery>(&(StaticPJDBConnectionUtil.lock().unwrap()).connection);

        match todos_completed_result {
            Ok(todos_completed) => {
                pj_info!("fetch_todos_order_by_state in type completed success!");
                todos.push(todos_completed);
            }
            Err(e) => {
                pj_error!("fetch_todos_order_by_state in type completed failure {}", e);
                hasError = true;
                error = e;
            }
        }

        let todos_overdue_result = schema::todo::table
            .filter(state.eq(3))
            .load::<ToDoQuery>(&(StaticPJDBConnectionUtil.lock().unwrap()).connection);

        match todos_overdue_result {
            Ok(todos_overdue) => {
                pj_info!("fetch_todos_order_by_state in type overdue success!");
                todos.push(todos_overdue);
            }
            Err(e) => {
                pj_error!("fetch_todos_order_by_state in type overdue failure {}", e);
                hasError = true;
                error = e;
            }
        }

        if !hasError {
            Ok(todos)
        } else {
            Err(error)
        }
    }

    fn update_overdue_todos(&self) -> Result<Vec<ToDoQuery>, diesel::result::Error> {
        let sql = format!("UPDATE todo set state = 3 WHERE due_time < date('now') and state == 0",);

        let result = sql_query(sql)
            .load::<ToDoQuery>(&(StaticPJDBConnectionUtil.lock().unwrap()).connection);
        match result {
            Ok(update_rows) => {
                pj_info!("update_overdue_todos success!");
                Ok(update_rows)
            }
            Err(e) => {
                pj_error!("update_overdue_todos failure {}", e);
                Err(e)
            }
        }
    }
}

pub fn createPJToDoDAOImpl() -> impl PJToDoDAO {
    PJToDoDAOImpl {}
}
