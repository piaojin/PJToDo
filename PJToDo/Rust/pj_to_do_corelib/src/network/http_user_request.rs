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
use mine::user::User;
use mine::authorizations::Authorizations;
use common::pj_utils::PJUtils;
use delegates::to_do_http_request_delegate::{IPJToDoHttpRequestDelegateWrapper, IPJToDoHttpRequestDelegate};
use std::ffi::{CStr, CString};
use libc::{c_char};

pub struct PJHttpUserRequest;

impl PJHttpUserRequest {
    pub fn request_user_info<F>(completion_handler: F)
    where
        F: FnOnce(Result<User, FetchError>)
            + std::marker::Sync
            + Send
            + 'static
            + std::clone::Clone,
    {
        let request = PJHttpRequest::default_request(PJRequestConfig::user_info());
        PJHttpRequest::http_send(request, |result| match result {
            Ok(body) => {
                let parse_result = PJHttpRequest::parse_data::<User>(&body);
                let result = match parse_result {
                    Ok(model) => Ok(model),
                    Err(e) => Err(FetchError::Json(e)),
                };
                completion_handler(result);
            }
            Err(e) => {
                completion_handler(Err(e));
            }
        });
    }

    pub fn login<'a, F>(name: &'a str, password: &'a str, completion_handler: F)
    where
        F: FnOnce(Result<User, FetchError>)
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

        PJHttpRequest::http_send(request, |result| match result {
            Ok(body) => {
                let parse_result = PJHttpRequest::parse_data::<User>(&body);
                let result = match parse_result {
                    Ok(model) => Ok(model),
                    Err(e) => Err(FetchError::Json(e)),
                };
                completion_handler(result);
            }
            Err(e) => {
                completion_handler(Err(e));
            }
        });
    }

    pub fn authorizations<'a, F>(authorization: &'a str, completion_handler: F)
    where
        F: FnOnce(Result<Authorizations, FetchError>)
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

        PJHttpRequest::http_send(request, |result| match result {
            Ok(body) => {
                let parse_result = PJHttpRequest::parse_data::<Authorizations>(&body);
                let result = match parse_result {
                    Ok(model) => Ok(model),
                    Err(e) => Err(FetchError::Json(e)),
                };
                completion_handler(result);
            }
            Err(e) => {
                completion_handler(Err(e));
            }
        });
    }
}

#[no_mangle]
pub unsafe extern "C" fn PJ_Login(
    delegate: IPJToDoHttpRequestDelegate,
    name: *const c_char,
    password: *const c_char,
) {
    if name.is_null() || password.is_null() {
        pj_error!("name or password: *mut PJ_Login is null!");
        assert!(!name.is_null() && !password.is_null());
    }

    let i_delegate = IPJToDoHttpRequestDelegateWrapper(delegate);
    let name = CStr::from_ptr(name).to_string_lossy().into_owned();
    let password = CStr::from_ptr(password).to_string_lossy().into_owned();

    PJHttpUserRequest::login(&name, &password, move |result| {
        let mut c_str = CString::new("").unwrap();
        match result {
            Ok(user) => {
                pj_info!("user: {:?}", user);
                // Serialize it to a JSON string.
                let json_string_result = serde_json::to_string(&user);
                match json_string_result {
                    Ok(json_string) => {
                        c_str = CString::new(json_string).unwrap();
                        let c_char = c_str.into_raw();
                        (i_delegate.request_result)(i_delegate.user, c_char, true);
                    }
                    Err(e) => {
                        pj_error!("PJ_Login request parse error: {:?}", e);
                        c_str = CString::new("{error: PJ_Login parse user to json data error!}")
                            .unwrap();
                        let c_char = c_str.into_raw();
                        (i_delegate.request_result)(i_delegate.user, c_char, true);
                    }
                }
            }
            Err(e) => {
                pj_error!("PJ_Login request error: {:?}", e);
                let c_char = c_str.into_raw();
                (i_delegate.request_result)(i_delegate.user, c_char, false);
            }
        }
    });
}

#[no_mangle]
pub unsafe extern "C" fn PJ_Authorizations(
    delegate: IPJToDoHttpRequestDelegate,
    authorization: *const c_char,
) {
    if authorization.is_null() {
        pj_error!("authorization: *mut PJ_Authorizations is null!");
        assert!(!authorization.is_null());
    }

    let i_delegate = IPJToDoHttpRequestDelegateWrapper(delegate);
    let authorization = CStr::from_ptr(authorization).to_string_lossy().into_owned();

    PJHttpUserRequest::authorizations(&authorization, move |result| {
        let mut c_str = CString::new("").unwrap();
        match result {
            Ok(authorization) => {
                pj_info!("authorization: {:?}", authorization);
                // Serialize it to a JSON string.
                let json_string_result = serde_json::to_string(&authorization);
                match json_string_result {
                    Ok(json_string) => {
                        c_str = CString::new(json_string).unwrap();
                        let c_char = c_str.into_raw();
                        (i_delegate.request_result)(i_delegate.user, c_char, true);
                    }
                    Err(e) => {
                        pj_error!("PJ_Authorizations request parse error: {:?}", e);
                        c_str = CString::new(
                            "{error: PJ_Authorizations parse authorization to json data error!}",
                        )
                        .unwrap();
                        let c_char = c_str.into_raw();
                        (i_delegate.request_result)(i_delegate.user, c_char, true);
                    }
                }
            }
            Err(e) => {
                pj_error!("PJ_Authorizations request error: {:?}", e);
                let c_char = c_str.into_raw();
                (i_delegate.request_result)(i_delegate.user, c_char, false);
            }
        }
    });
}

#[no_mangle]
pub unsafe extern "C" fn PJ_Request_user_info(delegate: IPJToDoHttpRequestDelegate) {
    let i_delegate = IPJToDoHttpRequestDelegateWrapper(delegate);

    PJHttpUserRequest::request_user_info(move |result| {
        let mut c_str = CString::new("").unwrap();
        match result {
            Ok(user) => {
                pj_info!("user: {:?}", user);
                // Serialize it to a JSON string.
                let json_string_result = serde_json::to_string(&user);
                match json_string_result {
                    Ok(json_string) => {
                        c_str = CString::new(json_string).unwrap();
                        let c_char = c_str.into_raw();
                        (i_delegate.request_result)(i_delegate.user, c_char, true);
                    }
                    Err(e) => {
                        pj_error!("PJ_Request_user_info request parse error: {:?}", e);
                        c_str = CString::new(
                            "{error: PJ_Request_user_info parse user to json data error!}",
                        )
                        .unwrap();
                        let c_char = c_str.into_raw();
                        (i_delegate.request_result)(i_delegate.user, c_char, true);
                    }
                }
            }
            Err(e) => {
                pj_error!("PJ_Request_user_info request error: {:?}", e);
                let c_char = c_str.into_raw();
                (i_delegate.request_result)(i_delegate.user, c_char, false);
            }
        }
    });
}
