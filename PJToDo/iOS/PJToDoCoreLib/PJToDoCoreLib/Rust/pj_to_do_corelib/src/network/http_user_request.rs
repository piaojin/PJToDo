extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate tokio;
extern crate rustc_serialize;
extern crate serde;
extern crate serde_json;

use self::rustc_serialize::base64::{STANDARD, ToBase64};

use self::hyper::header::HeaderValue;
use self::hyper::Method;
use crate::network::http_request::{PJHttpRequest, FetchError};
use crate::common::request_config::PJRequestConfig;
use crate::common::utils::pj_utils::PJUtils;

pub struct PJHttpUserRequest;

impl PJHttpUserRequest {
    pub fn request_user_info<F>(completion_handler: F)
    where
        F: FnOnce(Result<(hyper::StatusCode, String), FetchError>)
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
        F: FnOnce(Result<(hyper::StatusCode, String), FetchError>)
            + std::marker::Sync
            + Send
            + 'static
            + std::clone::Clone,
    {
        let request = PJHttpRequest::default_request(PJRequestConfig::login());

        let authorization_str = format!("{}:{}", name, password);
        let config = STANDARD;
        let authorization = authorization_str.as_bytes().to_base64(config);
        let _basic: &'static str =
            PJUtils::string_to_static_str(format!("Basic {}", authorization));

        pj_info!("👉👉The Resuest headers are: {:?}👈👈", request.headers());
        PJHttpRequest::make_http(request, completion_handler);
    }

    /// login with access_token, access_token will set in make_http fn.
    pub fn login_via_access_token<F>(completion_handler: F)
    where
        F: FnOnce(Result<(hyper::StatusCode, String), FetchError>)
            + std::marker::Sync
            + Send
            + 'static
            + std::clone::Clone,
    {
        let request = PJHttpRequest::default_request(PJRequestConfig::login());
        PJHttpRequest::make_http(request, completion_handler);
    }

    //auth token will create once and can't create again need to delete it in github.
    pub fn authorizations<'a, F>(authorization: &'a str, completion_handler: F)
    where
        F: FnOnce(Result<(hyper::StatusCode, String), FetchError>)
            + std::marker::Sync
            + Send
            + 'static
            + std::clone::Clone,
    {
        let authorization = PJUtils::string_to_static_str(authorization.to_string());

        let mut request = PJHttpRequest::request_with(
            PJRequestConfig::authorizations(),
            PJRequestConfig::authorization_body(),
            Method::POST,
        );

        request.headers_mut().insert(
            PJRequestConfig::authorization_head(),
            HeaderValue::from_static(authorization),
        );

        PJHttpRequest::make_http(request, completion_handler);
    }

    //auth token will create once and can't create again need to delete it in github.
    pub fn access_token<'a, F>(
        code: &'a str,
        client_id: &'a str,
        client_secret: &'a str,
        completion_handler: F,
    ) where
        F: FnOnce(Result<(hyper::StatusCode, String), FetchError>)
            + std::marker::Sync
            + Send
            + 'static
            + std::clone::Clone,
    {
        let mut body: String = format!(
            "\"client_id\":\"{}\",\"client_secret\":\"{}\",\"code\":\"{}\"",
            client_id, client_secret, code
        );
        body.insert_str(0, "{");
        body.push_str("}");

        let request = PJHttpRequest::request_with(
            PJRequestConfig::access_token_api(),
            &PJUtils::string_to_static_str(body),
            Method::POST,
        );

        PJHttpRequest::make_http(request, completion_handler);
    }
}
