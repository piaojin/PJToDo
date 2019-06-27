extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate tokio;
extern crate rustc_serialize;
extern crate serde;
extern crate serde_json;

use self::rustc_serialize::base64::{STANDARD, ToBase64};

use self::hyper::header::{HeaderValue};
use self::hyper::{Method};
use network::http_request::{PJHttpRequest, FetchError};
use common::request_config::PJRequestConfig;
use common::utils::pj_utils::{PJUtils};

pub struct PJHttpUserRequest;

impl PJHttpUserRequest {
    pub fn request_user_info<F>(completion_handler: F)
    where
        F: FnOnce(Result<(hyper::StatusCode, hyper::Chunk), FetchError>)
            + std::marker::Sync
            + Send
            + 'static
            + std::clone::Clone,
    {
        let request = PJHttpRequest::default_request(PJRequestConfig::user_info());
        PJHttpRequest::make_http(request, completion_handler);
    }

    pub fn login<'a, F>(name: &'a str, password: &'a str, completion_handler: F)
    where
        F: FnOnce(Result<(hyper::StatusCode, hyper::Chunk), FetchError>)
            + std::marker::Sync
            + Send
            + 'static
            + std::clone::Clone,
    {
        let mut request = PJHttpRequest::default_request(PJRequestConfig::login());

        let authorization_str = format!("{}:{}", name, password);
        let config = STANDARD;
        let authorization = authorization_str.as_bytes().to_base64(config);
        let basic: &'static str = PJUtils::string_to_static_str(format!("Basic {}", authorization));

        request.headers_mut().insert(
            PJRequestConfig::authorization_head(),
            HeaderValue::from_static(basic),
        );

        PJHttpRequest::make_http(request, completion_handler);
    }

    //auth token will create once and can't create again need to delete it in github.
    pub fn authorizations<'a, F>(authorization: &'a str, completion_handler: F)
    where
        F: FnOnce(Result<(hyper::StatusCode, hyper::Chunk), FetchError>)
            + std::marker::Sync
            + Send
            + 'static
            + std::clone::Clone,
    {
        let authorization = PJUtils::string_to_static_str(authorization.to_string());

        let mut request = PJHttpRequest::request_with(
            PJRequestConfig::authorizations(),
            PJRequestConfig::authorization_body(),
        );

        *request.method_mut() = Method::POST;
        request.headers_mut().insert(
            PJRequestConfig::authorization_head(),
            HeaderValue::from_static(authorization),
        );

        PJHttpRequest::make_http(request, completion_handler);
    }
}
