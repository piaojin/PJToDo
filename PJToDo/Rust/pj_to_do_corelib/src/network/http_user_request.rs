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
use common::pj_utils::{PJUtils, PJHttpUtils};
use delegates::to_do_http_request_delegate::{IPJToDoHttpRequestDelegateWrapper, IPJToDoHttpRequestDelegate};
use std::ffi::{CStr};
use libc::{c_char};
use std::thread;
use common::pj_model_utils::PJModelUtils;
use mine::user::User;

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

#[no_mangle]
pub unsafe extern "C" fn PJ_Login(
    delegate: IPJToDoHttpRequestDelegate,
    name: *const c_char,
    password: *const c_char,
) {
    if name == std::ptr::null_mut() || password == std::ptr::null_mut() {
        pj_error!("name or password: *mut PJ_Login is null!");
        assert!(name != std::ptr::null_mut() && password != std::ptr::null_mut());
    }

    let i_delegate = IPJToDoHttpRequestDelegateWrapper(delegate);
    let name = CStr::from_ptr(name).to_string_lossy().into_owned();
    let password = CStr::from_ptr(password).to_string_lossy().into_owned();

    thread::spawn(move || {
        PJHttpUserRequest::login(&name, &password, move |result| {
            let result = PJModelUtils::update_user_with_result(result);
            PJHttpRequest::dispatch_http_response(result, i_delegate);
        });
    });
}

#[no_mangle]
pub unsafe extern "C" fn PJ_Authorizations(
    delegate: IPJToDoHttpRequestDelegate,
    authorization: *const c_char,
) {
    if authorization == std::ptr::null_mut() {
        pj_error!("authorization: *mut PJ_Authorizations is null!");
        assert!(authorization != std::ptr::null_mut());
    }

    let i_delegate = IPJToDoHttpRequestDelegateWrapper(delegate);
    let authorization = CStr::from_ptr(authorization).to_string_lossy().into_owned();

    thread::spawn(move || {
        PJHttpUserRequest::authorizations(&authorization, move |result| {
            PJHttpRequest::dispatch_http_response(result, i_delegate);
        });
    });
}

#[no_mangle]
pub unsafe extern "C" fn PJ_RequestUserInfo(delegate: IPJToDoHttpRequestDelegate) {
    let i_delegate = IPJToDoHttpRequestDelegateWrapper(delegate);

    thread::spawn(move || {
        PJHttpUserRequest::request_user_info(move |result| {
            PJHttpRequest::dispatch_http_response(result, i_delegate);
        });
    });
}
