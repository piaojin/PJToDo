extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate tokio;
extern crate rustc_serialize;

use self::hyper::{Method, Request};

use crate::network::http_request::{PJHttpRequest, FetchError};
use crate::common::utils::pj_utils::PJUtils;
use crate::repos::repos_file::ReposFileBody;
use crate::network;
use crate::delegates::to_do_http_request_delegate::IPJToDoHttpRequestDelegateWrapper;
use std::ffi::CString;

#[derive(PartialEq, Debug)]
pub enum FileActionType {
    Create,
    Update,
    Delete,
    Get,
}

pub struct PJHttpReposFileRequest;

//repos file
impl PJHttpReposFileRequest {
    pub fn create_repos_file<F>(
        request_url: String,
        create_file_request_body: ReposFileBody,
        completion_handler: F,
    ) where
        F: FnOnce(Result<(hyper::StatusCode, String), FetchError>)
            + std::marker::Sync
            + Send
            + 'static
            + std::clone::Clone,
    {
        PJHttpReposFileRequest::crud_repos_file(
            request_url,
            create_file_request_body,
            FileActionType::Create,
            completion_handler,
        );
    }

    pub fn update_repos_file<F>(
        request_url: String,
        create_file_request_body: ReposFileBody,
        completion_handler: F,
    ) where
        F: FnOnce(Result<(hyper::StatusCode, String), FetchError>)
            + std::marker::Sync
            + Send
            + 'static
            + std::clone::Clone,
    {
        PJHttpReposFileRequest::crud_repos_file(
            request_url,
            create_file_request_body,
            FileActionType::Update,
            completion_handler,
        );
    }

    pub fn delete_repos_file<F>(
        request_url: String,
        create_file_request_body: ReposFileBody,
        completion_handler: F,
    ) where
        F: FnOnce(Result<(hyper::StatusCode, String), FetchError>)
            + std::marker::Sync
            + Send
            + 'static
            + std::clone::Clone,
    {
        PJHttpReposFileRequest::crud_repos_file(
            request_url,
            create_file_request_body,
            FileActionType::Delete,
            completion_handler,
        );
    }

    pub fn get_repos_file<F>(request_url: String, completion_handler: F)
    where
        F: FnOnce(Result<(hyper::StatusCode, String), FetchError>)
            + std::marker::Sync
            + Send
            + 'static
            + std::clone::Clone,
    {
        unsafe {
            let path = CString::new("PJToDo/Data/pj_to_db.zip".to_string()).unwrap();
            let message = CString::new("message".to_string()).unwrap();
            let content = CString::new("content".to_string()).unwrap();
            let sha = CString::new("sha".to_string()).unwrap();
            PJHttpReposFileRequest::crud_repos_file(
                request_url,
                ReposFileBody::new(
                    path.into_raw(),
                    message.into_raw(),
                    content.into_raw(),
                    sha.into_raw(),
                ),
                FileActionType::Get,
                completion_handler,
            );
        }
    }

    pub fn download_file<F>(request_url: String, completion_handler: F)
    where
        F: FnOnce(Result<(hyper::StatusCode, String), FetchError>)
            + std::marker::Sync
            + Send
            + 'static
            + std::clone::Clone,
    {
        let mut request = PJHttpRequest::default_request(&request_url);
        *request.method_mut() = Method::GET;
        PJHttpRequest::make_http(request, completion_handler);
    }

    fn crud_repos_file<F>(
        request_url: String,
        file_request_body: ReposFileBody,
        file_action_type: FileActionType,
        completion_handler: F,
    ) where
        F: FnOnce(Result<(hyper::StatusCode, String), FetchError>)
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
                    PJHttpReposFileRequest::crud_repos_file_request(
                        request_url,
                        file_request_body,
                        file_action_type,
                        completion_handler,
                    );
                }
            } else {
                PJHttpReposFileRequest::crud_repos_file_request(
                    request_url,
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

    fn crud_repos_file_request<F>(
        request_url: String,
        file_request_body: ReposFileBody,
        file_action_type: FileActionType,
        completion_handler: F,
    ) where
        F: FnOnce(Result<(hyper::StatusCode, String), FetchError>)
            + std::marker::Sync
            + Send
            + 'static
            + std::clone::Clone,
    {
        let update_file_api_url = &request_url;
        let repos_request_body_json =
            PJUtils::string_to_static_str(file_request_body.to_json_string());

        let mut request_method = Method::PUT;

        if file_action_type == FileActionType::Delete {
            request_method = Method::DELETE;
        } else if file_action_type == FileActionType::Get {
            request_method = Method::GET;
        }

        let request = PJHttpRequest::request_with(
            &update_file_api_url,
            &repos_request_body_json,
            request_method,
        );

        PJHttpReposFileRequest::do_crud_repos_file_request(request, completion_handler);
    }

    fn do_crud_repos_file_request<F>(request: Request<String>, completion_handler: F)
    where
        F: FnOnce(Result<(hyper::StatusCode, String), FetchError>)
            + std::marker::Sync
            + Send
            + 'static
            + std::clone::Clone,
    {
        PJHttpRequest::make_http(request, completion_handler);
    }

    pub fn dispatch_file_response(
        i_delegate: IPJToDoHttpRequestDelegateWrapper,
        result: Result<(hyper::StatusCode, String), FetchError>,
        request_action_name: &str,
    ) {
        pj_info!("request_action_name: {}", request_action_name);
        PJHttpRequest::dispatch_http_response(result, i_delegate);
    }
}
