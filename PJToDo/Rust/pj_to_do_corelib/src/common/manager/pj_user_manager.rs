extern crate hyper;
use std::sync::Mutex;
use mine::user::User;
use common::pj_serialize::PJSerdeDeserialize;
use network::http_request::FetchError;
use common::utils::pj_utils::{PJHttpUtils};
use common::manager::pj_repos_manager::PJReposManager;
use common::manager::pj_repos_file_manager::PJReposFileManager;

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

    pub fn update_user_with_result(result: Result<(hyper::StatusCode, hyper::Chunk), FetchError>) -> Result<(hyper::StatusCode, hyper::Chunk), FetchError> {
        match result {
            Ok((status, body)) => {
                if status.is_success() {
                    let parse_result = PJHttpUtils::parse_data::<User>(&body);
                    match parse_result {
                        Ok(user) => {
                            PJUserManager::update_user(user);
                        },
                        Err(_) => {}
                    };
                }
                Ok((status, body))
            },
            Err(_) => {
                result
            }
        }
    }

    pub fn logout() {
        PJUserManager::remove_user();
        PJReposFileManager::remove_repos_file();
        PJReposManager::remove_repos();
    }
}