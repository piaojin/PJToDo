extern crate hyper;
use std::sync::Mutex;
use crate::repos::repos_content::{ReposFile, ReposContent};
use crate::common::pj_serialize::{PJSerializeUtils, PJSerdeDeserialize};
use crate::network::http_request::FetchError;

lazy_static! {
    pub static ref REPOSFILE: Mutex<ReposFile> = Mutex::new(ReposFile::new());
}

pub struct PJReposFileManager;

impl PJReposFileManager {
    pub fn update_repos_file(reposFile: ReposFile) {
        match REPOSFILE.lock() {
            Ok(mut info) => {
                *info = reposFile;
            }

            Err(_) => {}
        }
    }

    pub fn remove_repos_file() {
        PJReposFileManager::update_repos_file(ReposFile::new());
    }

    pub fn update_repos_file_with_result(
        result: Result<(hyper::StatusCode, String), FetchError>,
    ) -> Result<(hyper::StatusCode, String), FetchError> {
        match result {
            Ok((status, body)) => {
                if status.is_success() {
                    let parse_result = PJSerializeUtils::from_str::<ReposFile>(&body);
                    match parse_result {
                        Ok(repos_file) => {
                            PJReposFileManager::update_repos_file(repos_file);
                        }
                        Err(err) => {
                            pj_error!(
                                "❌❌❌❌❌❌parse repos file result failed: {}❌❌❌❌❌❌",
                                err
                            );
                        }
                    };
                }
                Ok((status, body))
            }
            Err(_) => result,
        }
    }

    pub fn update_repos_file_content_with_result(
        result: Result<(hyper::StatusCode, String), FetchError>,
    ) -> Result<(hyper::StatusCode, String), FetchError> {
        match result {
            Ok((status, body)) => {
                if status.is_success() {
                    let parse_result = PJSerializeUtils::from_str::<ReposContent>(&body);
                    match parse_result {
                        Ok(repos_content) => {
                            let mut repos_file = ReposFile::new();
                            repos_file.set_content(repos_content);
                            PJReposFileManager::update_repos_file(repos_file);
                        }
                        Err(err) => {
                            pj_error!(
                                "❌❌❌❌❌❌parse repos file result failed: {}❌❌❌❌❌❌",
                                err
                            );
                        }
                    };
                }
                Ok((status, body))
            }
            Err(_) => result,
        }
    }
}
