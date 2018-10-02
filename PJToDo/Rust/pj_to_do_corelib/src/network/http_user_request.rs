extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate tokio;
extern crate rustc_serialize;

use self::rustc_serialize::base64::{STANDARD, ToBase64};

use self::hyper::header::{HeaderValue};
use self::hyper::{Method};

use network::http_request::{PJHttpRequest, FetchError};
use common::request_config::PJRequestConfig;
use mine::user::User;
use mine::authorizations::Authorizations;
use common::pj_utils::PJUtils;

pub struct PJHttpUserRequest;

impl PJHttpUserRequest {

    pub fn request_user_info<F>(completion_handler: F) 
    where F: FnOnce(Result<User, FetchError>) + std::marker::Sync + Send + 'static + std::clone::Clone {
        let request = PJHttpRequest::default_request(PJRequestConfig::user_info());
        PJHttpRequest::http_send(request, |result| {
            match result {
                Ok(body) => {
                    let parse_result = PJHttpRequest::parse_data::<User>(&body);
                    let result = match parse_result {
                        Ok(model) => Ok(model),
                        Err(e) => Err(FetchError::Json(e)),
                    };
                    completion_handler(result);
                },
                Err(e) => {
                    completion_handler(Err(e));
                }
            }
        });
    }

    pub fn login<'a, F>(name: &'a str, password: &'a str, completion_handler: F) 
    where F: FnOnce(Result<User, FetchError>) + std::marker::Sync + Send + 'static + std::clone::Clone {
        let mut request = PJHttpRequest::default_request(PJRequestConfig::login());

        let authorization_str = format!("{}:{}", name, password);
        let config = STANDARD;
        let authorization = authorization_str.as_bytes().to_base64(config);
        let basic: &'static str = PJUtils::string_to_static_str(format!("Basic {}", authorization));
        
        request.headers_mut().insert(
            PJRequestConfig::authorization_head(),
            HeaderValue::from_static(basic)
        );

        PJHttpRequest::http_send(request, |result| {
            match result {
                Ok(body) => {
                    let parse_result = PJHttpRequest::parse_data::<User>(&body);
                    let result = match parse_result {
                        Ok(model) => Ok(model),
                        Err(e) => Err(FetchError::Json(e)),
                    };
                    completion_handler(result);
                },
                Err(e) => {
                    completion_handler(Err(e));
                }
            }
        });
    }

    pub fn authorizations<'a, F>(authorization: &'a str, completion_handler: F) 
    where F: FnOnce(Result<Authorizations, FetchError>) + std::marker::Sync + Send + 'static + std::clone::Clone {
        
        let authorization = PJUtils::string_to_static_str(authorization.to_string());

        let mut request = PJHttpRequest::request_with(PJRequestConfig::authorizations(), PJRequestConfig::authorization_body());
        
        *request.method_mut() = Method::POST;
        request.headers_mut().insert(
            PJRequestConfig::authorization_head(),
            HeaderValue::from_static(authorization)
        );

        PJHttpRequest::http_send(request, |result| {
            match result {
                Ok(body) => {
                    let parse_result = PJHttpRequest::parse_data::<Authorizations>(&body);
                    let result = match parse_result {
                        Ok(model) => Ok(model),
                        Err(e) => Err(FetchError::Json(e)),
                    };
                    completion_handler(result);
                },
                Err(e) => {
                    completion_handler(Err(e));
                }
            }
        });
    }
}