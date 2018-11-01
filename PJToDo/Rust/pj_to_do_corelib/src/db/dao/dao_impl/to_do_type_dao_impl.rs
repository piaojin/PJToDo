use to_do_type::to_do_type::{ToDoTypeInsert, ToDoType};
use db::dao::to_do_type_dao::PJToDoTypeDAO;
use db::pj_db_connection_util::pj_db_connection_util::StaticPJDBConnectionUtil;
use db::tables::schema;
use diesel::prelude::*;
use common::pj_logger::PJLogger;
use std::ffi::CString;
use db::tables::schema::todotype::dsl::*;

pub struct PJToDoTypeDAOImpl;

// unsafe impl Sync for PJToDoTypeDAOImpl {}

impl PJToDoTypeDAO for PJToDoTypeDAOImpl {
    // add code here
    /**
     * 添加分类
     */
    fn insert_to_do_type(
        &self,
        to_do_type: &ToDoTypeInsert,
    ) -> Result<usize, diesel::result::Error> {
        pj_info!("insert_to_do_type: to_do_type: {:?}", to_do_type);
        let inserted_result = diesel::insert_into(schema::todotype::table)
            .values(to_do_type)
            .execute(&StaticPJDBConnectionUtil.connection);
        match inserted_result {
            Ok(inserted_row) => {
                let mut result: Result<usize, diesel::result::Error> =
                    Err(diesel::result::Error::NotFound);;
                if inserted_row == 1 {
                    result = Ok(inserted_row);
                } else {
                    pj_error!("insert_to_do_type: insert failure!");
                }
                pj_info!("insert_to_do_type: inserted_row: {}", inserted_row);
                result
            }
            Err(e) => {
                pj_error!("insert_to_do_type: insert failure {}!", e);
                Err(e)
            }
        }
    }

    fn delete_to_do_type(&self, to_do_type_id: i32) -> Result<usize, diesel::result::Error> {
        let deleted_result = diesel::delete(todotype.filter(id.eq(to_do_type_id)))
            .execute(&StaticPJDBConnectionUtil.connection);
        match deleted_result {
            Ok(deleted_row) => {
                let mut result: Result<usize, diesel::result::Error> =
                    Err(diesel::result::Error::NotFound);
                if deleted_row == 1 {
                    result = Ok(deleted_row);
                } else {
                    pj_error!("delete_to_do_type: delete failure!");
                }
                pj_info!("delete_to_do_type: delete_row: {}", deleted_row);
                result
            }
            Err(e) => {
                pj_error!("delete_to_do_type: delete failure {}", e);
                Err(e)
            }
        }
    }

    fn update_to_do_type(&self, to_do_type: &ToDoType) -> Result<usize, diesel::result::Error> {
        pj_info!("insert_to_do_type: to_do_type: {:?}", to_do_type);
        let update_result = diesel::update(todotype.filter(id.eq(to_do_type.id)))
            .set(to_do_type)
            .execute(&StaticPJDBConnectionUtil.connection);

        match update_result {
            Ok(update_row) => {
                let mut result: Result<usize, diesel::result::Error> =
                    Err(diesel::result::Error::NotFound);
                if update_row == 1 {
                    result = Ok(update_row);
                } else {
                    pj_error!("update_to_do_type: update failure!");
                }
                pj_info!("update_to_do_type: update_row: {}", update_row);
                result
            }
            Err(e) => {
                pj_error!("update_to_do_type: update failure {}", e);
                Err(e)
            }
        }
    }

    fn find_to_do_type_by_id(&self, to_do_type_id: i32) -> Result<ToDoType, diesel::result::Error> {
        pj_info!("find_to_do_type_by_id: to_do_type_id: {:?}", to_do_type_id);
        let find_result = todotype
            .find(to_do_type_id)
            .first::<ToDoType>(&StaticPJDBConnectionUtil.connection);
        match find_result {
            Ok(result) => {
                pj_info!("find_to_do_type_by_id success!");
                Ok(result)
            }
            Err(e) => {
                pj_error!("find_to_do_type_by_id failure {}", e);
                Err(e)
            }
        }
    }

    fn find_to_do_type_by_name(&self, name: String) -> Result<ToDoType, diesel::result::Error> {
        pj_info!("find_to_do_type_by_name: to_do_type_id: {}", name);

        let to_do_types_result = schema::todotype::table
            .filter(type_name.eq(name))
            .load::<ToDoType>(&StaticPJDBConnectionUtil.connection);

        match to_do_types_result {
            Ok(to_do_types) => {
                pj_info!("find_to_do_type_by_name success!");
                let first_to_do_type = to_do_types.first();
                match first_to_do_type {
                    Some(to_do_type) => {
                        let type_name_str: &str = &(to_do_type.type_name);
                        let todo_type = ToDoType {
                            id: to_do_type.id,
                            type_name: String::from(type_name_str),
                        };
                        Ok(todo_type)
                    }
                    None => Err(diesel::result::Error::NotFound),
                }
            }
            Err(e) => {
                pj_error!("find_to_do_type_by_name failure {}", e);
                Err(e)
            }
        }
    }
}

pub fn createPJToDoTypeDAOImpl() -> impl PJToDoTypeDAO {
    PJToDoTypeDAOImpl {}
}
