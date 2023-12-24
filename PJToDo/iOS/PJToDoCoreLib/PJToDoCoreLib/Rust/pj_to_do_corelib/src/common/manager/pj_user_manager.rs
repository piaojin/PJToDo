extern crate hyper;
use std::sync::Mutex;
use crate::mine::user::User;
use crate::network::http_request::FetchError;
use crate::common::manager::pj_repos_manager::PJReposManager;
use crate::common::manager::pj_repos_file_manager::PJReposFileManager;
use crate::common::pj_serialize::{PJSerializeUtils, PJSerdeDeserialize};

lazy_static! {
    pub static ref USERINFO: Mutex<User> = Mutex::new(User::new());
}

pub struct PJUserManager;

impl PJUserManager {
    pub fn update_user(user: User) {
        *(USERINFO.lock().unwrap()) = user;
    }

    pub fn remove_user() {
        PJUserManager::update_user(User::new());
    }

    pub fn update_user_with_result(
        result: Result<(hyper::StatusCode, String), FetchError>,
    ) -> Result<(hyper::StatusCode, String), FetchError> {
        match result {
            Ok((status, body)) => {
                if status.is_success() {
                    let parse_result = PJSerializeUtils::from_str::<User>(&body);
                    match parse_result {
                        Ok(user) => {
                            PJUserManager::update_user(user);
                        }
                        Err(_) => {}
                    };
                }
                Ok((status, body))
            }
            Err(_) => result,
        }
    }

    pub fn logout() {
        PJUserManager::remove_user();
        PJReposFileManager::remove_repos_file();
        PJReposManager::remove_repos();
    }
}
