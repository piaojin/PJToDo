use to_do_tag::to_do_tag::{ToDoTagInsert, ToDoTag};
use db::dao::to_do_tag_dao::PJToDoTagDAO;
use db::pj_db_connection_util::pj_db_connection_util::StaticPJDBConnectionUtil;
use db::tables::schema;
use diesel::prelude::*;
#[allow(unused_imports)]
use common::pj_logger::PJLogger;
use db::tables::schema::todotag::dsl::*;

pub struct PJToDoTagDAOImpl;

impl PJToDoTagDAO for PJToDoTagDAOImpl {
    fn insert_todo_tag(&self, to_do_tag: &ToDoTagInsert) -> Result<usize, diesel::result::Error> {
        pj_info!("insert_todo_tag: to_do_tag: {:?}", to_do_tag);
        let inserted_result = diesel::insert_into(schema::todotag::table)
            .values(to_do_tag)
            .execute(&(StaticPJDBConnectionUtil.lock().unwrap()).connection);
        match inserted_result {
            Ok(inserted_row) => {
                let mut result: Result<usize, diesel::result::Error> =
                    Err(diesel::result::Error::NotFound);;
                if inserted_row == 1 {
                    result = Ok(inserted_row);
                } else {
                    pj_error!("insert_todo_tag: insert failure!");
                }
                pj_info!("insert_todo_tag: inserted_row: {}", inserted_row);
                result
            }
            Err(e) => {
                pj_error!("insert_todo_tag: insert failure {}!", e);
                Err(e)
            }
        }
    }

    fn delete_todo_tag(&self, to_do_tag_id: i32) -> Result<usize, diesel::result::Error> {
        let deleted_result = diesel::delete(todotag.filter(id.eq(to_do_tag_id)))
            .execute(&(StaticPJDBConnectionUtil.lock().unwrap()).connection);
        match deleted_result {
            Ok(deleted_row) => {
                let mut result: Result<usize, diesel::result::Error> =
                    Err(diesel::result::Error::NotFound);
                if deleted_row == 1 {
                    result = Ok(deleted_row);
                } else {
                    pj_error!("delete_todo_tag: delete failure!");
                }
                pj_info!("delete_todo_tag: delete_row: {}", deleted_row);
                result
            }
            Err(e) => {
                pj_error!("delete_todo_tag: delete failure {}", e);
                Err(e)
            }
        }
    }

    fn update_todo_tag(&self, to_do_tag: &ToDoTag) -> Result<usize, diesel::result::Error> {
        pj_info!("update_todo_tag: to_do_tag: {:?}", to_do_tag);
        let update_result = diesel::update(todotag.filter(id.eq(to_do_tag.id)))
            .set(to_do_tag)
            .execute(&(StaticPJDBConnectionUtil.lock().unwrap()).connection);

        match update_result {
            Ok(update_row) => {
                let mut result: Result<usize, diesel::result::Error> =
                    Err(diesel::result::Error::NotFound);
                if update_row == 1 {
                    result = Ok(update_row);
                } else {
                    pj_error!("update_todo_tag: update failure!");
                }
                pj_info!("update_todo_tag: update_row: {}", update_row);
                result
            }
            Err(e) => {
                pj_error!("update_todo_tag: update failure {}", e);
                Err(e)
            }
        }
    }

    fn find_todo_tag_by_id(&self, to_do_tag_id: i32) -> Result<ToDoTag, diesel::result::Error> {
        pj_info!("find_todo_tag_by_id: to_do_tag_id: {:?}", to_do_tag_id);
        let find_result = todotag
            .find(to_do_tag_id)
            .first::<ToDoTag>(&(StaticPJDBConnectionUtil.lock().unwrap()).connection);
        match find_result {
            Ok(result) => {
                pj_info!("find_todo_tag_by_id success!");
                Ok(result)
            }
            Err(e) => {
                pj_error!("find_todo_tag_by_id failure {}", e);
                Err(e)
            }
        }
    }

    fn find_todo_tag_by_name(&self, name: String) -> Result<ToDoTag, diesel::result::Error> {
        pj_info!("find_todo_tag_by_name: to_do_tag_id: {}", name);

        let to_do_tags_result = schema::todotag::table
            .filter(tag_name.eq(name))
            .load::<ToDoTag>(&(StaticPJDBConnectionUtil.lock().unwrap()).connection);

        match to_do_tags_result {
            Ok(to_do_tags) => {
                pj_info!("find_todo_tag_by_name success!");
                let first_todo_tag = to_do_tags.first();
                match first_todo_tag {
                    Some(to_do_tag) => {
                        let tag_name_str: &str = &(to_do_tag.tag_name);
                        let todo_type = ToDoTag {
                            id: to_do_tag.id,
                            tag_name: String::from(tag_name_str),
                        };
                        Ok(todo_type)
                    }
                    None => Err(diesel::result::Error::NotFound),
                }
            }
            Err(e) => {
                pj_error!("find_todo_tag_by_name failure {}", e);
                Err(e)
            }
        }
    }

    fn fetch_data(&self) -> Result<Vec<ToDoTag>, diesel::result::Error> {
        let to_do_tags_result = todotag.load::<ToDoTag>(&(StaticPJDBConnectionUtil.lock().unwrap()).connection);
        match to_do_tags_result {
            Ok(to_do_tags) => {
                pj_info!("fetchData success!");
                Ok(to_do_tags)
            }
            Err(e) => {
                pj_error!("fetchData failure {}", e);
                Err(e)
            }
        }
    }
}

pub fn createPJToDoTagDAOImpl() -> impl PJToDoTagDAO {
    PJToDoTagDAOImpl {}
}
