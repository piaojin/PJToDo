extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate tokio;

extern crate serde_derive;
extern crate serde;
extern crate serde_json;

// use self::serde::{Deserialize, Serialize};

use self::hyper_tls::HttpsConnector;
use self::hyper::{Request, Body, Method, Client, Uri};
use self::hyper::header::{HeaderValue};
use self::hyper::rt::{Stream, Future as OtherFuture};

#[allow(unused_imports)]
use common::pj_logger::PJLogger;
use common::pj_serialize::PJSerdeDeserialize;

use pal::pj_user_pal_help::PJUserPALHelp;
use common::pj_utils::PJUtils;

// Define a type so we can return multile types of errors
#[derive(Debug)]
pub enum FetchError {
    Http(hyper::Error),
    Json(serde_json::Error),
    Custom(String),
}

impl From<hyper::Error> for FetchError {
    fn from(err: hyper::Error) -> FetchError {
        FetchError::Http(err)
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
                    "Authorization",
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
    pub fn do_request_action(
        request: Request<Body>,
    ) -> impl OtherFuture<Item = hyper::Chunk, Error = FetchError> {
        // 4 is number of blocking DNS threads
        //为了使用https请求，默认是http请求
        let https = HttpsConnector::new(4).unwrap();
        // https.https_only(true);
        let client = Client::builder().build::<_, hyper::Body>(https);

        client
            .request(request)
            .and_then(|res| {
                pj_info!("Headers: {:#?}", res.headers());
                pj_info!("Response: {:#?}", res.status());
                pj_info!("Body: {:#?}", res.body());
                if !res.status().is_success() {
                    pj_error!("request {:#?} faild!!!", res);
                } else {
                    pj_info!("request {:#?}", res);
                }
                //使用该函数则是把body字节数据返回
                res.into_body().concat2()
            })
            .from_err::<FetchError>()
            // use the body after concatenation
            .and_then(|body| Ok(body))
            .from_err()
    }

    pub fn parse_data<'a, T>(body: &'a hyper::Chunk) -> Result<T, serde_json::Error>
    where
        T: std::fmt::Debug + PJSerdeDeserialize<'a>,
    {
        // try to parse as json with serde_json
        let parse_result = serde_json::from_slice(body);
        pj_info!("parse data result: {:?}", parse_result);
        parse_result
    }

    pub fn http_send<F>(request: Request<Body>, completion_handler: F)
    where
        F: FnOnce(Result<hyper::Chunk, FetchError>)
            + std::marker::Sync
            + Send
            + 'static
            + std::clone::Clone,
    {
        let completion_handler_http_err = completion_handler.clone();
        let completion_handler_json_parse_err = completion_handler.clone();

        let response_data = PJHttpRequest::do_request_action(request)
            .map(|body: hyper::Chunk| body)
            // if there was an error print it
            .map_err(|e| {
                match e {
                    FetchError::Http(e) => {
                        // eprintln!("http error: {}", e)
                        completion_handler_http_err(Err(FetchError::Http(e)));
                    }
                    FetchError::Json(e) => {
                        // eprintln!("json parsing error: {}", e)
                        completion_handler_json_parse_err(Err(FetchError::Json(e)));
                    }
                    FetchError::Custom(e) => {
                        completion_handler_json_parse_err(Err(FetchError::Custom(e)));
                    }
                }
            })
            .map(|body: hyper::Chunk| {
                completion_handler(Ok(body));
            })
            // if there was an error print it
            .map_err(|e| eprintln!("request error: {:?}", e));

        hyper::rt::run(response_data);
    }
}
