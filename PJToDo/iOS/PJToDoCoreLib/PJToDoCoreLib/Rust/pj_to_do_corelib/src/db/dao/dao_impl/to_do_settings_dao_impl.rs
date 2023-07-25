use mine::todo_settings::{ToDoSettingsInsert, ToDoSettings};
use db::dao::to_do_settings_dao::PJToDoSettingsDAO;
use db::pj_db_connection_util::pj_db_connection_util::StaticPJDBConnectionUtil;
use db::tables::schema;
use diesel::prelude::*;
#[allow(unused_imports)]
use common::pj_logger::PJLogger;
use db::tables::schema::todosettings::dsl::*;

pub struct PJToDoSettingsDAOImpl;

impl PJToDoSettingsDAO for PJToDoSettingsDAOImpl {
    fn insert_todo_settings(
        &self,
        to_do_settings: &ToDoSettingsInsert,
    ) -> Result<usize, diesel::result::Error> {
        pj_info!("insert_todo_settings: to_do_settings: {:?}", to_do_settings);
        let inserted_result = diesel::insert_into(schema::todosettings::table)
            .values(to_do_settings)
            .execute(&(StaticPJDBConnectionUtil.lock().unwrap()).connection);
        match inserted_result {
            Ok(inserted_row) => {
                let mut result: Result<usize, diesel::result::Error> =
                    Err(diesel::result::Error::NotFound);
                if inserted_row == 1 {
                    result = Ok(inserted_row);
                } else {
                    pj_error!("insert_todo_settings: insert failure!");
                }
                pj_info!("insert_todo_settings: inserted_row: {}", inserted_row);
                result
            }
            Err(e) => {
                pj_error!("insert_todo_settings: insert failure {}!", e);
                Err(e)
            }
        }
    }

    fn delete_todo_settings(&self, to_do_settings_id: i32) -> Result<usize, diesel::result::Error> {
        let deleted_result = diesel::delete(todosettings.filter(id.eq(to_do_settings_id)))
            .execute(&(StaticPJDBConnectionUtil.lock().unwrap()).connection);
        match deleted_result {
            Ok(deleted_row) => {
                let mut result: Result<usize, diesel::result::Error> =
                    Err(diesel::result::Error::NotFound);
                if deleted_row == 1 {
                    result = Ok(deleted_row);
                } else {
                    pj_error!("delete_todo_settings: delete failure!");
                }
                pj_info!("delete_todo_settings: delete_row: {}", deleted_row);
                result
            }
            Err(e) => {
                pj_error!("delete_todo_settings: delete failure {}", e);
                Err(e)
            }
        }
    }

    fn update_todo_settings(
        &self,
        to_do_settings: &ToDoSettings,
    ) -> Result<usize, diesel::result::Error> {
        pj_info!("insert_todo_settings: to_do_settings: {:?}", to_do_settings);
        let update_result = diesel::update(todosettings.filter(id.eq(to_do_settings.id)))
            .set(to_do_settings)
            .execute(&(StaticPJDBConnectionUtil.lock().unwrap()).connection);

        match update_result {
            Ok(update_row) => {
                let mut result: Result<usize, diesel::result::Error> =
                    Err(diesel::result::Error::NotFound);
                if update_row == 1 {
                    result = Ok(update_row);
                } else {
                    pj_error!("update_todo_settings: update failure!");
                }
                pj_info!("update_todo_settings: update_row: {}", update_row);
                result
            }
            Err(e) => {
                pj_error!("update_todo_settings: update failure {}", e);
                Err(e)
            }
        }
    }

    fn fetch_data(&self) -> Result<Vec<ToDoSettings>, diesel::result::Error> {
        let to_do_settings_result = todosettings
            .load::<ToDoSettings>(&(StaticPJDBConnectionUtil.lock().unwrap()).connection);
        match to_do_settings_result {
            Ok(to_do_settingss) => {
                pj_info!("fetchData success!");
                Ok(to_do_settingss)
            }
            Err(e) => {
                pj_error!("fetchData failure {}", e);
                Err(e)
            }
        }
    }
}

pub fn createPJToDoSettingsDAOImpl() -> impl PJToDoSettingsDAO {
    PJToDoSettingsDAOImpl {}
}
