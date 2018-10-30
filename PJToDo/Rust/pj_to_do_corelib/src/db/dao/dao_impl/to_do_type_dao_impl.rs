use to_do_type::to_do_type::ToDoTypeInsert;
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
                let mut result: Result<usize, diesel::result::Error> = Err(
                    diesel::result::Error::from(CString::new("insert failure!").unwrap_err()),
                );;
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
                    Err(diesel::result::Error::from(
                        CString::new("delete_to_do_type: delete failure!").unwrap_err(),
                    ));
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
}

pub fn createPJToDoTypeDAOImpl() -> impl PJToDoTypeDAO {
    PJToDoTypeDAOImpl {}
}
