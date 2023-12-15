extern crate hyper;
use std::sync::Mutex;
use repos::repos_content::{ReposFile, ReposContent};
use common::pj_serialize::{PJSerializeUtils, PJSerdeDeserialize};
use network::http_request::FetchError;

lazy_static! {
    pub static ref REPOSFILE: Mutex<ReposFile> = Mutex::new(ReposFile::new());
}

pub struct PJReposFileManager;

impl PJReposFileManager {
    pub fn update_repos_file(reposFile: ReposFile) {
        *(REPOSFILE.lock().unwrap()) = reposFile;
    }

    pub fn remove_repos_file() {
        PJReposFileManager::update_repos_file(ReposFile::new());
    }

    pub fn update_repos_file_with_result(
        result: Result<(hyper::StatusCode, hyper::Chunk), FetchError>,
    ) -> Result<(hyper::StatusCode, hyper::Chunk), FetchError> {
        match result {
            Ok((status, body)) => {
                if status.is_success() {
                    let parse_result = PJSerializeUtils::from_hyper_chunk::<ReposFile>(&body);
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
        result: Result<(hyper::StatusCode, hyper::Chunk), FetchError>,
    ) -> Result<(hyper::StatusCode, hyper::Chunk), FetchError> {
        match result {
            Ok((status, body)) => {
                if status.is_success() {
                    let parse_result = PJSerializeUtils::from_hyper_chunk::<ReposContent>(&body);
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
