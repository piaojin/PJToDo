extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate tokio;
extern crate http_body_util;
extern crate hyper_util;

extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use self::hyper_tls::HttpsConnector;
use self::hyper::{Request, Method, Uri};
use self::hyper::header::HeaderValue;
use self::hyper_util::{client::legacy::Client, rt::TokioExecutor};
use self::http_body_util::BodyExt;

use crate::pal::pj_user_pal_help::PJUserPALHelp;
use crate::common::utils::pj_utils::PJUtils;
use crate::common::request_config::PJRequestConfig;
use std::ffi::CString;
use crate::delegates::to_do_http_request_delegate::IPJToDoHttpRequestDelegateWrapper;

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
    pub fn default_request(url: &str) -> Request<String> {
        let body = r#"{"library":"hyper"}"#;
        PJHttpRequest::request_with(url, body, Method::GET)
    }

    pub fn request_with(url: &str, body: &str, http_method: Method) -> Request<String> {
        pj_info!("👉👉Resuest Url: {}👈👈", url);
        let uri = url.parse::<Uri>();
        let mut req: Request<String>;
        if http_method != Method::GET {
            req = Request::new(body.to_owned());
        } else {
            req = Request::default();
        }

        match uri {
            Ok(uri) => {
                *req.method_mut() = http_method;
                *req.uri_mut() = uri.clone();

                req.headers_mut().insert(
                    hyper::header::ACCEPT,
                    HeaderValue::from_static("application/vnd.github+json"),
                );

                req.headers_mut().insert(
                    hyper::header::CONTENT_TYPE,
                    HeaderValue::from_static("application/json"),
                );

                req.headers_mut()
                    .insert(hyper::header::USER_AGENT, HeaderValue::from_static("hyper"));

                let access_token: &'static str = unsafe {
                    let mut access_token_str: String = PJUserPALHelp::get_access_token_str();
                    if !((&access_token_str).is_empty()) {
                        access_token_str.insert_str(0, "Bearer ");
                    } else {
                        pj_warn!("********Access_token is empty!!!*********");
                    }
                    PJUtils::string_to_static_str(access_token_str)
                };

                if !access_token.is_empty() {
                    req.headers_mut().insert(
                        PJRequestConfig::authorization_head(),
                        HeaderValue::from_static(access_token),
                    );
                }

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
    pub fn make_http<F>(request: Request<String>, completion_handler: F)
    where
        F: FnOnce(Result<(hyper::StatusCode, String), FetchError>)
            + std::marker::Sync
            + Send
            + 'static
            + std::clone::Clone,
    {
        let rt_res = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build();
        match rt_res {
            Ok(rt) => {
                rt.block_on(async move {
                    let url = request.uri().clone();
                    let method = request.method().clone();
                    let headers = request.headers().clone();
                    let req_body = request.body().clone();
                    PJHttpRequest::do_http_send(request, move |result| match result {
                        Ok((status, body)) => match String::from_utf8(body) {
                            Ok(body_string) => {
                                pj_info!("ℹ️ℹ️ℹ️ℹ️ℹ️ℹ️API {:#?} \n Method: {:#?} \n Headers: {:#?} \n Request Body: {:#?} \n Response Body: {:#?}ℹ️ℹ️ℹ️ℹ️ℹ️ℹ️", url, method, headers, req_body, body_string);
                                completion_handler(Ok((status, body_string)));
                            }

                            Err(e) => {
                                completion_handler(Err(FetchError::Custom(format!(
                                    "Cannot convert Vec<u8> response body to String: {}",
                                    e
                                ))));
                            }
                        },
                        Err(e) => {
                            completion_handler(Err(e));
                        }
                    })
                    .await
                });
            }

            Err(err) => {
                completion_handler(Err(FetchError::Custom(format!(
                    "Cannot create tokio::runtime::Builder: {}",
                    err
                ))));
            }
        }
    }

    pub fn make_http_with_raw_res_data<F>(request: Request<String>, completion_handler: F)
    where
        F: FnOnce(Result<(hyper::StatusCode, Vec<u8>), FetchError>)
            + std::marker::Sync
            + Send
            + 'static
            + std::clone::Clone,
    {
        let rt_res = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build();
        match rt_res {
            Ok(rt) => {
                rt.block_on(async move {
                    let url = request.uri().clone();
                    let method = request.method().clone();
                    let headers = request.headers().clone();
                    let req_body = request.body().clone();
                    PJHttpRequest::do_http_send(request, move |result| match result {
                        Ok((status, body)) => {
                            pj_info!(
                                "ℹ️ℹ️ℹ️ℹ️ℹ️ℹ️API {:#?} \n Method: {:#?} \n Headers: {:#?} \n Request Body: {:#?} \nℹ️ℹ️ℹ️ℹ️ℹ️ℹ️",
                                url,
                                method,
                                headers,
                                req_body
                            );
                            completion_handler(Ok((status, body)));
                        }
                        Err(e) => {
                            completion_handler(Err(e));
                        }
                    })
                    .await
                });
            }

            Err(err) => {
                completion_handler(Err(FetchError::Custom(format!(
                    "Cannot create tokio::runtime::Builder: {}",
                    err
                ))));
            }
        }
    }

    pub fn dispatch_http_response(
        result: Result<(hyper::StatusCode, String), FetchError>,
        i_delegate: IPJToDoHttpRequestDelegateWrapper,
    ) {
        match result {
            Ok((status, body)) => {
                let c_str = CString::new(body).unwrap_or(CString::default());
                let c_char = c_str.into_raw();
                (i_delegate.request_result)(
                    i_delegate.user,
                    c_char,
                    status.as_u16(),
                    status.is_success(),
                );
            }
            Err(e) => {
                let error_string: String;
                let mut error_code: u16 = 0;
                match e {
                    FetchError::Http(status, error) => {
                        error_string = error.to_string();
                        error_code = status.as_u16();
                    }
                    FetchError::Json(error) => {
                        error_string = format!(
                            "❌❌❌❌❌❌parser json error: {:?}!!!❌❌❌❌❌❌❌",
                            error
                        );
                    }
                    FetchError::Custom(custom_error_str) => {
                        error_string = custom_error_str;
                    }
                }
                let c_char = PJUtils::create_cstring_from(&error_string).into_raw();
                (i_delegate.request_result)(i_delegate.user, c_char, error_code, false);
            }
        };
    }

    pub async fn do_http_send<F>(request: Request<String>, completion_handler: F)
    where
        F: FnOnce(Result<(hyper::StatusCode, Vec<u8>), FetchError>)
            + std::marker::Sync
            + Send
            + 'static
            + std::clone::Clone,
    {
        let url = request.uri().clone();
        // 4 is number of blocking DNS threads
        //为了使用https请求，默认是http请求
        let https = HttpsConnector::new();
        // https.https_only(true);
        let client = Client::builder(TokioExecutor::new()).build::<_, String>(https);

        pj_info!("🙏🙏🙏🙏🙏🙏The Resuest is: {:?}🙏🙏🙏🙏🙏🙏", request);

        let res = client.request(request).await;

        match res {
            Ok(mut response) => {
                if response.status().is_success() {
                    pj_info!(
                        "✅✅✅✅✅✅API {:#?}, Response {:#?}✅✅✅✅✅✅",
                        url.clone(),
                        response
                    );
                } else {
                    pj_error!(
                        "❌❌❌❌❌❌API {:#?}, Response {:#?} faild!!!❌❌❌❌❌❌",
                        url.clone(),
                        response
                    );
                }

                let mut body_result_bytes: Vec<u8> = Vec::new();
                let mut is_response_ok: bool = true;
                let mut response_err: FetchError = FetchError::Custom("Default error".to_string());
                // response.body_mut().frame().await result is [Frame] and need loop to get result data.
                while let Some(next) = response.body_mut().frame().await {
                    match next {
                        Ok(frame) => {
                            if let Some(d) = frame.data_ref() {
                                body_result_bytes.append(&mut (d as &[u8]).to_owned());
                            }
                        }

                        Err(err) => {
                            is_response_ok = false;
                            response_err = FetchError::Custom(format!(
                                "Cannot get Frame<Bytes> from response body: {}",
                                err
                            ));
                            pj_error!("❌❌❌❌❌❌Err Response body:\n{:#?}❌❌❌❌❌❌", err);
                        }
                    }
                }

                if is_response_ok {
                    (completion_handler.clone())(Ok((response.status(), body_result_bytes)));
                } else {
                    (completion_handler.clone())(Err(response_err));
                }
            }

            Err(err) => {
                completion_handler(Err(FetchError::Custom(format!("Invalid request: {}", err))));
                pj_error!("❌❌❌❌❌❌Err Request:\n{:#?}❌❌❌❌❌❌", err);
            }
        };
    }
}
