extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate tokio;
extern crate rustc_serialize;

use self::hyper::header::{HeaderValue};
use self::hyper::{Method, Request, Body};

use network::http_request::{PJHttpRequest, FetchError};
use common::request_config::PJRequestConfig;
use common::pj_utils::PJUtils;
use repos::repos::{ReposRequestBody, Repos};
use repos::repos_file::ReposFileBody;
use repos::repos_content::{ReposFile};
#[allow(unused_imports)]
use common::pj_logger::PJLogger;
use network;
use common::pj_serialize::PJSerdeDeserialize;
use delegates::to_do_http_request_delegate::{IPJToDoHttpRequestDelegateWrapper, IPJToDoHttpRequestDelegate};
use std::ffi::{CStr, CString};
use libc::{c_char, c_void};

#[derive(PartialEq, Debug)]
pub enum FileActionType {
    Create,
    Update,
    Delete,
}

pub struct PJHttpReposRequest;

//repos
impl PJHttpReposRequest {
    pub fn create_repos<F>(repos_request_body: ReposRequestBody, completion_handler: F)
    where
        F: FnOnce(Result<Repos, FetchError>)
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
                let mut request =
                    PJHttpRequest::request_with(PJRequestConfig::repos(), &repos_request_body_json);

                *request.method_mut() = Method::POST;
                request.headers_mut().insert(
                    PJRequestConfig::authorization_head(),
                    HeaderValue::from_static(PJRequestConfig::personal_token()),
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
        F: FnOnce(Result<Repos, FetchError>)
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
        F: FnOnce(Result<Repos, FetchError>)
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
            request.headers_mut().insert(
                PJRequestConfig::authorization_head(),
                HeaderValue::from_static(PJRequestConfig::personal_token()),
            );
            *request.method_mut() = Method::DELETE;
            PJHttpReposRequest::do_repos_request(request, |result| match result {
                Ok(_) => {
                    completion_handler(Ok(Repos::new()));
                }
                Err(e) => {
                    completion_handler(Err(e));
                }
            });
        }
    }

    fn do_repos_request<F>(request: Request<Body>, completion_handler: F)
    where
        F: FnOnce(Result<Repos, FetchError>)
            + std::marker::Sync
            + Send
            + 'static
            + std::clone::Clone,
    {
        PJHttpRequest::http_send(request, |result| match result {
            Ok(body) => {
                let parse_result = PJHttpRequest::parse_data::<Repos>(&body);
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

//repos file
impl PJHttpReposRequest {
    pub fn create_file<F>(create_file_request_body: ReposFileBody, completion_handler: F)
    where
        F: FnOnce(Result<ReposFile, FetchError>)
            + std::marker::Sync
            + Send
            + 'static
            + std::clone::Clone,
    {
        PJHttpReposRequest::crud_file(
            create_file_request_body,
            FileActionType::Create,
            completion_handler,
        );
    }

    pub fn update_file<F>(create_file_request_body: ReposFileBody, completion_handler: F)
    where
        F: FnOnce(Result<ReposFile, FetchError>)
            + std::marker::Sync
            + Send
            + 'static
            + std::clone::Clone,
    {
        PJHttpReposRequest::crud_file(
            create_file_request_body,
            FileActionType::Update,
            completion_handler,
        );
    }

    pub fn delete_file<F>(create_file_request_body: ReposFileBody, completion_handler: F)
    where
        F: FnOnce(Result<ReposFile, FetchError>)
            + std::marker::Sync
            + Send
            + 'static
            + std::clone::Clone,
    {
        PJHttpReposRequest::crud_file(create_file_request_body, FileActionType::Delete, |result| {
            match result {
                Ok(_) => {
                    completion_handler(Ok(ReposFile::new()));
                }
                Err(e) => {
                    completion_handler(Err(e));
                }
            }
        });
    }

    fn crud_file<F>(
        file_request_body: ReposFileBody,
        file_action_type: FileActionType,
        completion_handler: F,
    ) where
        F: FnOnce(Result<ReposFile, FetchError>)
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
                    PJHttpReposRequest::crud_file_request(
                        file_request_body,
                        file_action_type,
                        completion_handler,
                    );
                }
            } else {
                PJHttpReposRequest::crud_file_request(
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

    fn crud_file_request<F>(
        file_request_body: ReposFileBody,
        file_action_type: FileActionType,
        completion_handler: F,
    ) where
        F: FnOnce(Result<ReposFile, FetchError>)
            + std::marker::Sync
            + Send
            + 'static
            + std::clone::Clone,
    {
        let update_file_api_url = format!(
            "{}{}",
            PJRequestConfig::get_repos_contents_url(),
            file_request_body.path
        );

        let file_request_body = serde_json::to_string(&file_request_body);

        match file_request_body {
            Ok(file_request_body_json) => {
                let repos_request_body_json =
                    PJUtils::string_to_static_str(file_request_body_json.to_string());
                let mut request =
                    PJHttpRequest::request_with(&update_file_api_url, &repos_request_body_json);

                let mut request_method = Method::PUT;

                if file_action_type == FileActionType::Delete {
                    request_method = Method::DELETE;
                }

                *request.method_mut() = request_method;
                request.headers_mut().insert(
                    PJRequestConfig::authorization_head(),
                    HeaderValue::from_static(PJRequestConfig::personal_token()),
                );
                PJHttpReposRequest::do_crud_file_request(request, completion_handler);
            }
            Err(e) => {
                pj_error!(
                    "file_request_body: ReposFileBody can not convert to json: {}",
                    e
                );
                let err = network::http_request::FetchError::from(String::from(format!(
                    "file_request_body: ReposFileBody can not convert to json: {}",
                    e
                )));
                completion_handler(Err(err));
            }
        }
    }

    #[warn(dead_code)]
    fn do_crud_file_request<F>(request: Request<Body>, completion_handler: F)
    where
        F: FnOnce(Result<ReposFile, FetchError>)
            + std::marker::Sync
            + Send
            + 'static
            + std::clone::Clone,
    {
        PJHttpRequest::http_send(request, |result| match result {
            Ok(body) => {
                let parse_result = PJHttpRequest::parse_data::<ReposFile>(&body);
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
pub unsafe extern "C" fn PJ_CreateRepos(delegate: IPJToDoHttpRequestDelegate) {
    let i_delegate = IPJToDoHttpRequestDelegateWrapper(delegate);

    PJHttpReposRequest::create_repos(PJRequestConfig::repos_request_body(), move |result| {
        let mut c_str = CString::new("").unwrap();
        match result {
            Ok(repos) => {
                pj_info!("repos: {:?}", repos);
                // Serialize it to a JSON string.
                let json_string_result = serde_json::to_string(&repos);
                match json_string_result {
                    Ok(json_string) => {
                        c_str = CString::new(json_string).unwrap();
                        let c_char = c_str.into_raw();
                        (i_delegate.request_result)(i_delegate.user, c_char, true);
                    }
                    Err(e) => {
                        pj_error!("PJ_CreateRepos request parse error: {:?}", e);
                        c_str =
                            CString::new("{error: PJ_CreateRepos parse user to json data error!}")
                                .unwrap();
                        let c_char = c_str.into_raw();
                        (i_delegate.request_result)(i_delegate.user, c_char, true);
                    }
                }
            }
            Err(e) => {
                pj_error!("PJ_CreateRepos request error: {:?}", e);
            }
        }
    });
}

#[no_mangle]
pub unsafe extern "C" fn PJ_GetRepos(
    delegate: IPJToDoHttpRequestDelegate,
    repos_url: *const c_char,
) {
    if repos_url.is_null() {
        pj_error!("repos_url: *mut PJ_GetRepos is null!");
        assert!(!repos_url.is_null());
    }

    let i_delegate = IPJToDoHttpRequestDelegateWrapper(delegate);
    let repos_url = CStr::from_ptr(repos_url).to_string_lossy().into_owned();

    PJHttpReposRequest::get_repos(&repos_url, move |result| {
        let mut c_str = CString::new("").unwrap();
        match result {
            Ok(repos) => {
                pj_info!("repos: {:?}", repos);
                // Serialize it to a JSON string.
                let json_string_result = serde_json::to_string(&repos);
                match json_string_result {
                    Ok(json_string) => {
                        c_str = CString::new(json_string).unwrap();
                        let c_char = c_str.into_raw();
                        (i_delegate.request_result)(i_delegate.user, c_char, true);
                    }
                    Err(e) => {
                        pj_error!("PJ_GetRepos request parse error: {:?}", e);
                        c_str = CString::new("{error: PJ_GetRepos parse user to json data error!}")
                            .unwrap();
                        let c_char = c_str.into_raw();
                        (i_delegate.request_result)(i_delegate.user, c_char, true);
                    }
                }
            }
            Err(e) => {
                pj_error!("PJ_GetRepos request error: {:?}", e);
            }
        }
    });
}
