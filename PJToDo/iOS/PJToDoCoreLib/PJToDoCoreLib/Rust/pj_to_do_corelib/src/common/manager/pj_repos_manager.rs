extern crate hyper;
use std::sync::Mutex;
use repos::repos::Repos;
use common::pj_serialize::{PJSerializeUtils, PJSerdeDeserialize};
use network::http_request::FetchError;

lazy_static! {
    pub static ref REPOS: Mutex<Repos> = Mutex::new(Repos::new());
}

pub struct PJReposManager;

impl PJReposManager {
    pub fn update_repos(repos: Repos) {
        *(REPOS.lock().unwrap()) = repos;
    }

    pub fn remove_repos() {
        PJReposManager::update_repos(Repos::new());
    }

    pub fn update_repos_with_result(
        result: Result<(hyper::StatusCode, hyper::Chunk), FetchError>,
    ) -> Result<(hyper::StatusCode, hyper::Chunk), FetchError> {
        match result {
            Ok((status, body)) => {
                if status.is_success() {
                    let parse_result = PJSerializeUtils::from_hyper_chunk::<Repos>(&body);
                    match parse_result {
                        Ok(repos) => {
                            PJReposManager::update_repos(repos);
                        }
                        Err(_) => {}
                    };
                }
                Ok((status, body))
            }
            Err(_) => result,
        }
    }
}
