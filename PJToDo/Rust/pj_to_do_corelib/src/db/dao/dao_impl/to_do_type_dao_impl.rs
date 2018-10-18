use to_do_type::to_do_type::ToDoTypeInsert;
use db::dao::to_do_type_dao::PJToDoTypeDAO;
use db::pj_db_connection_util::pj_db_connection_util::StaticPJDBConnectionUtil;
use db::tables::schema;
use diesel::prelude::*;
use common::pj_logger::PJLogger;

pub struct PJToDoTypeDAOImpl;

// unsafe impl Sync for PJToDoTypeDAOImpl {}

impl PJToDoTypeDAO for PJToDoTypeDAOImpl {
    // add code here
    /**
     * 添加分类
     */
    fn insert_to_do_type(&self, to_do_type: ToDoTypeInsert) -> Result<usize, String> {
        pj_info!("insert_to_do_type: to_do_type: {:?}", to_do_type);
        let inserted_result = diesel::insert_into(schema::todotype::table)
            .values(&to_do_type)
            .execute(&StaticPJDBConnectionUtil.connection);
        match inserted_result {
            Ok(inserted_row) => {
                let mut result: Result<usize, String> = Err(String::from("insert failure!"));
                if inserted_row == 1 {
                    result = Ok(inserted_row);
                } else {
                    pj_error!("insert_to_do_type: insert failure!");
                }
                pj_info!("insert_to_do_type: inserted_row: {}", inserted_row);
                result
            }
            Err(_) => {
                pj_error!("insert_to_do_type: insert failure!");
                Err(String::from("insert failure!"))
            }
        }
    }
}
