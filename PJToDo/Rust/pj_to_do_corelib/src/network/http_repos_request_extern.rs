use network::http_repos_request::PJHttpReposRequest;
use delegates::to_do_http_request_delegate::{IPJToDoHttpRequestDelegateWrapper, IPJToDoHttpRequestDelegate};
use std::ffi::{CStr, CString};
use libc::{c_char};
use repos::repos::{Repos};
use repos::repos_file::ReposFileBody;
use repos::repos_content::{ReposFile};
use common::request_config::PJRequestConfig;
use network::http_request::{FetchError};

#[no_mangle]
pub unsafe extern "C" fn PJ_CreateRepos(delegate: IPJToDoHttpRequestDelegate) {
    let i_delegate = IPJToDoHttpRequestDelegateWrapper(delegate);

    PJHttpReposRequest::create_repos(PJRequestConfig::repos_request_body(), move |result| {
        dispatch_repos_request_action_result(i_delegate, result, "PJ_CreateRepos");
    });
}

#[no_mangle]
pub unsafe extern "C" fn PJ_GetRepos(
    delegate: IPJToDoHttpRequestDelegate,
    repos_url: *const c_char,
) {
    if repos_url == std::ptr::null_mut() {
        pj_error!("repos_url: *mut PJ_GetRepos is null!");
        assert!(repos_url != std::ptr::null_mut());
    }

    let i_delegate = IPJToDoHttpRequestDelegateWrapper(delegate);
    let repos_url = CStr::from_ptr(repos_url).to_string_lossy().into_owned();

    PJHttpReposRequest::get_repos(&repos_url, move |result| {
        dispatch_repos_request_action_result(i_delegate, result, "PJ_GetRepos");
    });
}

#[no_mangle]
pub unsafe extern "C" fn PJ_DeleteRepos(
    delegate: IPJToDoHttpRequestDelegate,
    repos_url: *const c_char,
) {
    if repos_url == std::ptr::null_mut() {
        pj_error!("repos_url: *mut PJ_DeleteRepos is null!");
        assert!(repos_url != std::ptr::null_mut());
    }

    let i_delegate = IPJToDoHttpRequestDelegateWrapper(delegate);
    let repos_url = CStr::from_ptr(repos_url).to_string_lossy().into_owned();

    PJHttpReposRequest::delete_repos(&repos_url, move |result| {
        dispatch_repos_request_action_result(i_delegate, result, "PJ_DeleteRepos");
    });
}

#[no_mangle]
pub unsafe extern "C" fn PJ_CreateFile(
    delegate: IPJToDoHttpRequestDelegate,
    path: *const c_char,
    message: *const c_char,
    content: *const c_char,
    sha: *const c_char,
) {
    if path == std::ptr::null_mut() || message == std::ptr::null_mut() || content == std::ptr::null_mut() || sha == std::ptr::null_mut() {
        pj_error!("path or message or content or sha: *mut PJ_CreateFile is null!");
        assert!(path != std::ptr::null_mut() && message != std::ptr::null_mut() && content != std::ptr::null_mut() && sha != std::ptr::null_mut());
    }

    let i_delegate = IPJToDoHttpRequestDelegateWrapper(delegate);
    let repos_file_body = ReposFileBody::new(path, message, content, sha);

    PJHttpReposRequest::create_file(repos_file_body, move |result| {
        dispatch_file_request_action_result(i_delegate, result, "PJ_CreateFile");
    });
}

#[no_mangle]
pub unsafe extern "C" fn PJ_UpdateFile(
    delegate: IPJToDoHttpRequestDelegate,
    path: *const c_char,
    message: *const c_char,
    content: *const c_char,
    sha: *const c_char,
) {
    if path == std::ptr::null_mut() || message == std::ptr::null_mut() || content == std::ptr::null_mut() || sha == std::ptr::null_mut() {
        pj_error!("path or message or content or sha: *mut PJ_UpdateFile is null!");
        assert!(path != std::ptr::null_mut() && message != std::ptr::null_mut() && content != std::ptr::null_mut() && sha != std::ptr::null_mut());
    }

    let i_delegate = IPJToDoHttpRequestDelegateWrapper(delegate);
    let repos_file_body = ReposFileBody::new(path, message, content, sha);

    PJHttpReposRequest::update_file(repos_file_body, move |result| {
        dispatch_file_request_action_result(i_delegate, result, "PJ_UpdateFile");
    });
}

#[no_mangle]
pub unsafe extern "C" fn PJ_DeleteFile(
    delegate: IPJToDoHttpRequestDelegate,
    path: *const c_char,
    message: *const c_char,
    content: *const c_char,
    sha: *const c_char,
) {
    if path == std::ptr::null_mut() || message == std::ptr::null_mut() || content == std::ptr::null_mut() || sha == std::ptr::null_mut() {
        pj_error!("path or message or content or sha: *mut PJ_UpdateFile is null!");
        assert!(path != std::ptr::null_mut() && message != std::ptr::null_mut() && content != std::ptr::null_mut() && sha != std::ptr::null_mut());
    }

    let i_delegate = IPJToDoHttpRequestDelegateWrapper(delegate);
    let repos_file_body = ReposFileBody::new(path, message, content, sha);

    PJHttpReposRequest::delete_file(repos_file_body, move |result| {
        dispatch_file_request_action_result(i_delegate, result, "PJ_DeleteFile");
    });
}

fn dispatch_repos_request_action_result(
    i_delegate: IPJToDoHttpRequestDelegateWrapper,
    result: Result<Repos, FetchError>,
    request_action_name: &str,
) {
    match result {
        Ok(repos) => {
            pj_info!("repos: {:?}", repos);
            // Serialize it to a JSON string.
            let json_string_result = serde_json::to_string(&repos);
            match json_string_result {
                Ok(json_string) => {
                    let c_str = CString::new(json_string).unwrap();
                    let c_char = c_str.into_raw();
                    (i_delegate.request_result)(i_delegate.user, c_char, true);
                }
                Err(e) => {
                    let error = format!("{} request parse error: {:?}", request_action_name, e);
                    pj_error!("{}", error);

                    let parse_error = format!(
                        "error: {} parse user to json data error!",
                        request_action_name
                    );
                    let c_str = CString::new(parse_error.to_string()).unwrap();
                    let c_char = c_str.into_raw();
                    (i_delegate.request_result)(i_delegate.user, c_char, true);
                }
            }
        }
        Err(e) => {
            let error = format!("{} request parse error: {:?}", request_action_name, e);
            pj_error!("{}", error);
        }
    };
}

fn dispatch_file_request_action_result(
    i_delegate: IPJToDoHttpRequestDelegateWrapper,
    result: Result<ReposFile, FetchError>,
    request_action_name: &str,
) {
    match result {
        Ok(repos) => {
            pj_info!("repos: {:?}", repos);
            // Serialize it to a JSON string.
            let json_string_result = serde_json::to_string(&repos);
            match json_string_result {
                Ok(json_string) => {
                    let c_str = CString::new(json_string).unwrap();
                    let c_char = c_str.into_raw();
                    (i_delegate.request_result)(i_delegate.user, c_char, true);
                }
                Err(e) => {
                    let error = format!("{} request parse error: {:?}", request_action_name, e);
                    pj_error!("{}", error);

                    let parse_error = format!(
                        "error: {} parse user to json data error!",
                        request_action_name
                    );
                    let c_str = CString::new(parse_error.to_string()).unwrap();
                    let c_char = c_str.into_raw();
                    (i_delegate.request_result)(i_delegate.user, c_char, true);
                }
            }
        }
        Err(e) => {
            let error = format!("{} request parse error: {:?}", request_action_name, e);
            pj_error!("{}", error);
        }
    };
}
