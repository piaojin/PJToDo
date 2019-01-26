extern crate hyper;
use std::sync::Mutex;
use mine::user::User;
use repos::repos::Repos;
use common::pj_serialize::PJSerdeDeserialize;
use network::http_request::FetchError;
use common::pj_utils::{PJHttpUtils};

lazy_static! {
    pub static ref USERINFO: Mutex<User> = Mutex::new(User::new());
    pub static ref REPOS: Mutex<Repos> = Mutex::new(Repos::new());
}

pub struct PJModelUtils;

impl PJModelUtils {
    pub fn update_user(user: User) {
        *(USERINFO.lock().unwrap()) = user;
    }

    pub fn update_user_with_result(result: Result<(hyper::StatusCode, hyper::Chunk), FetchError>) -> Result<(hyper::StatusCode, hyper::Chunk), FetchError> {
        match result {
            Ok((status, body)) => {
                if status.is_success() {
                    let parse_result = PJHttpUtils::parse_data::<User>(&body);
                    match parse_result {
                        Ok(user) => {
                            PJModelUtils::update_user(user);
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

    pub fn update_repos(repos: Repos) {
        *(REPOS.lock().unwrap()) = repos;
    }

    pub fn update_repos_with_result(result: Result<(hyper::StatusCode, hyper::Chunk), FetchError>) -> Result<(hyper::StatusCode, hyper::Chunk), FetchError> {
        match result {
            Ok((status, body)) => {
                if status.is_success() {
                    let parse_result = PJHttpUtils::parse_data::<Repos>(&body);
                    match parse_result {
                        Ok(repos) => {
                            PJModelUtils::update_repos(repos);
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
}