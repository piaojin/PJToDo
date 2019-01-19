use network::http_repos_request::PJHttpReposRequest;
use delegates::to_do_http_request_delegate::{IPJToDoHttpRequestDelegateWrapper, IPJToDoHttpRequestDelegate};
use std::ffi::{CStr};
use libc::{c_char};
use repos::repos_file::ReposFileBody;
use common::request_config::PJRequestConfig;
use std::thread;

#[no_mangle]
pub unsafe extern "C" fn PJ_CreateRepos(delegate: IPJToDoHttpRequestDelegate) {
    let i_delegate = IPJToDoHttpRequestDelegateWrapper(delegate);

    thread::spawn(move || {
        PJHttpReposRequest::create_repos(PJRequestConfig::repos_request_body(), move |result| {
            PJHttpReposRequest::dispatch_repos_response(i_delegate, result, "PJ_CreateRepos");
        });
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

    thread::spawn(move || {
        PJHttpReposRequest::get_repos(&repos_url, move |result| {
            PJHttpReposRequest::dispatch_repos_response(i_delegate, result, "PJ_GetRepos");
        });
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

    thread::spawn(move || {
        PJHttpReposRequest::delete_repos(&repos_url, move |result| {
            PJHttpReposRequest::dispatch_repos_response(i_delegate, result, "PJ_DeleteRepos");
        });
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

    thread::spawn(move || {
        PJHttpReposRequest::create_file(repos_file_body, move |result| {
            PJHttpReposRequest::dispatch_file_response(i_delegate, result, "PJ_CreateFile");
        }); 
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

    thread::spawn(move || {
        PJHttpReposRequest::update_file(repos_file_body, move |result| {
            PJHttpReposRequest::dispatch_file_response(i_delegate, result, "PJ_UpdateFile");
        });
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

    thread::spawn(move || {
        PJHttpReposRequest::delete_file(repos_file_body, move |result| {
            PJHttpReposRequest::dispatch_file_response(i_delegate, result, "PJ_DeleteFile");
        });
    });
}
