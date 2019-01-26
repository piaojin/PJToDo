extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate tokio;

extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use self::hyper_tls::HttpsConnector;
use self::hyper::{Request, Body, Method, Client, Uri};
use self::hyper::header::{HeaderValue};
use self::hyper::rt::{Stream, Future as OtherFuture};

#[allow(unused_imports)]
use common::pj_logger::PJLogger;

use pal::pj_user_pal_help::PJUserPALHelp;
use common::utils::pj_utils::{PJUtils, PJHttpUtils};
use common::request_config::PJRequestConfig;
use std::ffi::CString;
use delegates::to_do_http_request_delegate::IPJToDoHttpRequestDelegateWrapper;
use std::sync::{Arc, Mutex};

// Define a type so we can return multile types of errors
#[derive(Debug)]
pub enum FetchError {
    Http(hyper::StatusCode, hyper::Error),
    Json(serde_json::Error),
    Custom(String),
}

impl From<hyper::Error> for FetchError {
    fn from(err: hyper::Error) -> FetchError {
        FetchError::Http(hyper::StatusCode::OK, err)
    }
}

impl From<serde_json::Error> for FetchError {
    fn from(err: serde_json::Error) -> FetchError {
        FetchError::Json(err)
    }
}

impl From<String> for FetchError {
    fn from(err: String) -> FetchError {
        FetchError::Custom(err)
    }
}

#[derive(Debug)]
pub struct PJHttpRequest;

impl PJHttpRequest {
    pub fn default_request(url: &str) -> Request<Body> {
        let body = r#"{"library":"hyper"}"#;
        PJHttpRequest::request_with(url, body)
    }

    pub fn request_with(url: &str, body: &'static str) -> Request<Body> {
        let uri = url.parse::<Uri>();
        let mut req = Request::new(Body::from(body));
        match uri {
            Ok(uri) => {
                *req.method_mut() = Method::GET;
                *req.uri_mut() = uri.clone();
                req.headers_mut().insert(
                    hyper::header::CONTENT_LENGTH,
                    HeaderValue::from_static("application/json"),
                );

                req.headers_mut().insert(
                    hyper::header::ACCEPT,
                    HeaderValue::from_static("application/json"),
                );

                req.headers_mut().insert(
                    hyper::header::CONTENT_TYPE,
                    HeaderValue::from_static("application/json"),
                );

                req.headers_mut().insert(
                    hyper::header::USER_AGENT,
                    HeaderValue::from_static("application/vnd.github.v3+json"),
                );
                
                let authorization: &'static str = unsafe {
                    let user_authorization_str: String = PJUserPALHelp::get_user_authorization_str();
                    if (&user_authorization_str).is_empty() {
                        pj_error!("********Error authorization is empty!!!*********");
                    }
                    PJUtils::string_to_static_str(user_authorization_str)
                };

                req.headers_mut().insert(
                    PJRequestConfig::authorization_head(),
                    HeaderValue::from_static(authorization),
                );
                req
            }
            Err(e) => {
                pj_error!("url format error: {}!!!", e);
                req
            }
        }
    }
}

impl PJHttpRequest {

    pub fn make_http<F>(request: Request<Body>, completion_handler: F)
    where
        F: FnOnce(Result<(hyper::StatusCode, hyper::Chunk), FetchError>)
            + std::marker::Sync
            + Send
            + 'static
            + std::clone::Clone,
    {
        PJHttpRequest::do_http_send(request, |result| match result {
            Ok((status, body)) => {
                completion_handler(Ok((status, body)));
            }
            Err(e) => {
                completion_handler(Err(e));
            }
        });
    }

    pub fn dispatch_http_response(result: Result<(hyper::StatusCode, hyper::Chunk), FetchError>, i_delegate: IPJToDoHttpRequestDelegateWrapper) {
        match result {
            Ok((status, body)) => {
                let c_str = CString::new(PJHttpUtils::hyper_body_to_string(body)).unwrap();
                let c_char = c_str.into_raw();
                (i_delegate.request_result)(i_delegate.user, c_char, status.as_u16(), status.is_success());
            },
            Err(e) => {
                let mut error_string: String;
                let mut error_code: u16 = 0;
                match e {
                    FetchError::Http(status, error) => {
                        error_string = error.to_string();
                        error_code = status.as_u16();
                    }
                    FetchError::Json(error) => {
                        error_string = format!("❌parser json error: {:?}!!!❌", error);
                    }
                    FetchError::Custom(custom_error_str) => {
                        error_string = custom_error_str;
                    }
                }
                let c_char = CString::new(error_string).unwrap().into_raw();
                (i_delegate.request_result)(i_delegate.user, c_char, error_code, false);
            }
        };
    }

    pub fn do_http_send<F>(request: Request<Body>, completion_handler: F)
    where
        F: FnOnce(Result<(hyper::StatusCode, hyper::Chunk), FetchError>)
            + std::marker::Sync
            + Send
            + 'static
            + std::clone::Clone,
    {
        let completion_handler_http_err = completion_handler.clone();
        let completion_handler_json_parse_err = completion_handler.clone();

        // 4 is number of blocking DNS threads
        //为了使用https请求，默认是http请求
        let https = HttpsConnector::new(4).unwrap();
        // https.https_only(true);
        let client = Client::builder().build::<_, hyper::Body>(https);
        
        let status_code: hyper::StatusCode = hyper::StatusCode::OK;
        let status_map: Arc<Mutex<_>> = Arc::new(Mutex::new(status_code));
        let share_status_map_i = status_map.clone();
        let share_status_map_ii = status_map.clone();
        let share_status_map_iii = status_map.clone();

        let response = client
            .request(request)
            .and_then(move |res| {
                pj_info!("👉Response headers: {:#?}👈", res.headers());
                pj_info!("👉Response status code: {:#?}👈", res.status());

                let mut share_status_data = share_status_map_i.lock().unwrap();
                *share_status_data = res.status();

                pj_info!("👉Response body: {:#?}👈", res.body());
                if !res.status().is_success() {
                    pj_error!("❌Response {:#?} faild!!!❌", res);
                } else {
                    pj_info!("👉👉Response {:#?}👈👈", res);
                }
                //使用该函数则是把body字节数据返回
                res.into_body().concat2()
            })
            .from_err::<FetchError>()
            // use the body after concatenation
            .and_then(|body| Ok(body))
            .from_err();

        let response_data = response
            .map(|body: hyper::Chunk| body)
            // if there was an error print it
            .map_err(move |e| {
                match e {
                    FetchError::Http(_, e) => {
                        completion_handler_http_err(Err(FetchError::Http(*(share_status_map_ii.lock().unwrap()), e)));
                    }
                    FetchError::Json(e) => {
                        completion_handler_json_parse_err(Err(FetchError::Json(e)));
                    }
                    FetchError::Custom(e) => {
                        completion_handler_json_parse_err(Err(FetchError::Custom(e)));
                    }
                }
            })
            .map(move |body: hyper::Chunk| {
                completion_handler(Ok((*(share_status_map_iii.lock().unwrap()), body)));
            })
            // if there was an error print it
            .map_err(|e| {
                eprintln!("❌request error: {:?}❌", e);
            });

        hyper::rt::run(response_data);
    }
}
