extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate tokio;
extern crate rustc_serialize;

use self::hyper::{Method, Request, Body};

use network::http_request::{PJHttpRequest, FetchError};
use common::request_config::PJRequestConfig;
use common::pj_utils::PJUtils;
use repos::repos::{ReposRequestBody};
use repos::repos_file::ReposFileBody;
#[allow(unused_imports)]
use common::pj_logger::PJLogger;
use network;
use delegates::to_do_http_request_delegate::IPJToDoHttpRequestDelegateWrapper;

#[derive(PartialEq, Debug)]
pub enum FileActionType {
    Create,
    Update,
    Delete,
}

pub struct PJHttpReposRequest;

//repos
impl PJHttpReposRequest {
    pub fn create_repos<F>(repos_request_body: ReposRequestBody, completion_handler: F)
    where
        F: FnOnce(Result<(hyper::StatusCode, hyper::Chunk), FetchError>)
            + std::marker::Sync
            + Send
            + 'static
            + std::clone::Clone,
    {
        let repos_request_body = serde_json::to_string(&repos_request_body);

        match repos_request_body {
            Ok(repos_request_body_json) => {
                let repos_request_body_json =
                    PJUtils::string_to_static_str(repos_request_body_json.to_string());
                let mut request =
                    PJHttpRequest::request_with(PJRequestConfig::repos(), &repos_request_body_json);

                *request.method_mut() = Method::POST;

                PJHttpReposRequest::do_repos_request(request, completion_handler);
            }
            Err(e) => {
                pj_error!(
                    "parse repos_request_body: ReposRequestBody to json error: {}!!!",
                    e
                );
                let err = network::http_request::FetchError::from(String::from(format!(
                    "parse repos_request_body: ReposRequestBody to json error: {}!!!",
                    e
                )));
                completion_handler(Err(err));
            }
        }
    }

    pub fn get_repos<F>(repos_url: &str, completion_handler: F)
    where
        F: FnOnce(Result<(hyper::StatusCode, hyper::Chunk), FetchError>)
            + std::marker::Sync
            + Send
            + 'static
            + std::clone::Clone,
    {
        if repos_url.is_empty() {
            pj_error!("repos_url can not be empty!!!");
            let err = network::http_request::FetchError::from(String::from(
                "repos_url can not be empty!!!",
            ));
            completion_handler(Err(err));
        } else {
            let request = PJHttpRequest::default_request(repos_url);
            PJHttpReposRequest::do_repos_request(request, completion_handler);
        }
    }

    pub fn delete_repos<F>(repos_url: &str, completion_handler: F)
    where
        F: FnOnce(Result<(hyper::StatusCode, hyper::Chunk), FetchError>)
            + std::marker::Sync
            + Send
            + 'static
            + std::clone::Clone,
    {
        if repos_url.is_empty() {
            pj_error!("repos_url can not be empty!!!");
            let err = network::http_request::FetchError::from(String::from(
                "repos_url can not be empty!!!",
            ));
            completion_handler(Err(err));
        } else {
            let mut request = PJHttpRequest::default_request(repos_url);
            *request.method_mut() = Method::DELETE;
            PJHttpReposRequest::do_repos_request(request, completion_handler);
        }
    }

    fn do_repos_request<F>(request: Request<Body>, completion_handler: F)
    where
        F: FnOnce(Result<(hyper::StatusCode, hyper::Chunk), FetchError>)
            + std::marker::Sync
            + Send
            + 'static
            + std::clone::Clone, {
        PJHttpRequest::make_http(request, completion_handler);
    }

    pub fn dispatch_repos_response(i_delegate: IPJToDoHttpRequestDelegateWrapper, result: Result<(hyper::StatusCode, hyper::Chunk), FetchError>, request_action_name: &str,
    ) {
        pj_info!("request_action_name: {}", request_action_name);
        PJHttpRequest::dispatch_http_response(result, i_delegate);
    }
}

//repos file
impl PJHttpReposRequest {
    pub fn create_file<F>(create_file_request_body: ReposFileBody, completion_handler: F)
    where
        F: FnOnce(Result<(hyper::StatusCode, hyper::Chunk), FetchError>)
            + std::marker::Sync
            + Send
            + 'static
            + std::clone::Clone,
    {
        PJHttpReposRequest::crud_file(
            create_file_request_body,
            FileActionType::Create,
            completion_handler,
        );
    }

    pub fn update_file<F>(create_file_request_body: ReposFileBody, completion_handler: F)
    where
        F: FnOnce(Result<(hyper::StatusCode, hyper::Chunk), FetchError>)
            + std::marker::Sync
            + Send
            + 'static
            + std::clone::Clone,
    {
        PJHttpReposRequest::crud_file(
            create_file_request_body,
            FileActionType::Update,
            completion_handler,
        );
    }

    pub fn delete_file<F>(create_file_request_body: ReposFileBody, completion_handler: F)
    where
        F: FnOnce(Result<(hyper::StatusCode, hyper::Chunk), FetchError>)
            + std::marker::Sync
            + Send
            + 'static
            + std::clone::Clone,
    {
        PJHttpReposRequest::crud_file(create_file_request_body, FileActionType::Delete, completion_handler);
    }

    fn crud_file<F>(
        file_request_body: ReposFileBody,
        file_action_type: FileActionType,
        completion_handler: F,
    ) where
        F: FnOnce(Result<(hyper::StatusCode, hyper::Chunk), FetchError>)
            + std::marker::Sync
            + Send
            + 'static
            + std::clone::Clone,
    {
        if !file_request_body.path.is_empty() {
            if file_action_type != FileActionType::Create {
                if file_request_body.sha.is_empty() {
                    pj_error!("ReposFileBody sha can not be empty!!!");
                    let err = network::http_request::FetchError::from(String::from(
                        "ReposFileBody sha can not be empty",
                    ));
                    completion_handler(Err(err));
                } else {
                    PJHttpReposRequest::crud_file_request(
                        file_request_body,
                        file_action_type,
                        completion_handler,
                    );
                }
            } else {
                PJHttpReposRequest::crud_file_request(
                    file_request_body,
                    file_action_type,
                    completion_handler,
                );
            }
        } else {
            pj_error!("ReposFileBody path can not be empty!!!");
            let err = network::http_request::FetchError::from(String::from(
                "ReposFileBody path can not be empty",
            ));
            completion_handler(Err(err));
        }
    }

    fn crud_file_request<F>(
        file_request_body: ReposFileBody,
        file_action_type: FileActionType,
        completion_handler: F,
    ) where
        F: FnOnce(Result<(hyper::StatusCode, hyper::Chunk), FetchError>)
            + std::marker::Sync
            + Send
            + 'static
            + std::clone::Clone,
    {
        let update_file_api_url = format!(
            "{}{}",
            PJRequestConfig::get_repos_contents_url(),
            file_request_body.path
        );

        let file_request_body = serde_json::to_string(&file_request_body);

        match file_request_body {
            Ok(file_request_body_json) => {
                let repos_request_body_json =
                    PJUtils::string_to_static_str(file_request_body_json.to_string());
                let mut request =
                    PJHttpRequest::request_with(&update_file_api_url, &repos_request_body_json);

                let mut request_method = Method::PUT;

                if file_action_type == FileActionType::Delete {
                    request_method = Method::DELETE;
                }

                *request.method_mut() = request_method;
                PJHttpReposRequest::do_crud_file_request(request, completion_handler);
            }
            Err(e) => {
                pj_error!(
                    "file_request_body: ReposFileBody can not convert to json: {}",
                    e
                );
                let err = network::http_request::FetchError::from(String::from(format!(
                    "file_request_body: ReposFileBody can not convert to json: {}",
                    e
                )));
                completion_handler(Err(err));
            }
        }
    }

    fn do_crud_file_request<F>(request: Request<Body>, completion_handler: F)
    where
        F: FnOnce(Result<(hyper::StatusCode, hyper::Chunk), FetchError>)
            + std::marker::Sync
            + Send
            + 'static
            + std::clone::Clone,
    {
        PJHttpRequest::make_http(request, completion_handler);
    }

    pub fn dispatch_file_response(i_delegate: IPJToDoHttpRequestDelegateWrapper, result: Result<(hyper::StatusCode, hyper::Chunk), FetchError>, request_action_name: &str,
    ) {
        pj_info!("request_action_name: {}", request_action_name);
        PJHttpRequest::dispatch_http_response(result, i_delegate);
    }
}
