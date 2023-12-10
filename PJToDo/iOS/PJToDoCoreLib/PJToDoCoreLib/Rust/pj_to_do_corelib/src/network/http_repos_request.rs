extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate tokio;
extern crate rustc_serialize;

use self::hyper::{Method, Request, Body};

use network::http_request::{PJHttpRequest, FetchError};
use common::request_config::PJRequestConfig;
use common::utils::pj_utils::PJUtils;
use repos::repos::{ReposRequestBody};
#[allow(unused_imports)]
use common::pj_logger::PJLogger;
use network;
use delegates::to_do_http_request_delegate::IPJToDoHttpRequestDelegateWrapper;

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
                let mut request = PJHttpRequest::request_with(
                    PJRequestConfig::repos(),
                    &repos_request_body_json,
                    Method::POST,
                );

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
            + std::clone::Clone,
    {
        PJHttpRequest::make_http(request, completion_handler);
    }

    pub fn dispatch_repos_response(
        i_delegate: IPJToDoHttpRequestDelegateWrapper,
        result: Result<(hyper::StatusCode, hyper::Chunk), FetchError>,
        request_action_name: &str,
    ) {
        pj_info!("request_action_name: {}", request_action_name);
        PJHttpRequest::dispatch_http_response(result, i_delegate);
    }
}
