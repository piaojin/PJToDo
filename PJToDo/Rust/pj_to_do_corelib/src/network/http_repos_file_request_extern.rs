use network::http_repos_file_request::PJHttpReposFileRequest;
use delegates::to_do_http_request_delegate::{IPJToDoHttpRequestDelegateWrapper, IPJToDoHttpRequestDelegate};
use std::ffi::{CStr};
use libc::{c_char};
use repos::repos_file::ReposFileBody;
use std::thread;
use common::manager::pj_repos_file_manager::PJReposFileManager;

#[no_mangle]
pub unsafe extern "C" fn PJ_CreateReposFile(
    delegate: IPJToDoHttpRequestDelegate,
    request_url: *const c_char,
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
    let request_url = CStr::from_ptr(request_url).to_string_lossy().into_owned();
    let repos_file_body = ReposFileBody::new(path, message, content, sha);

    thread::spawn(move || {
        PJHttpReposFileRequest::create_repos_file(request_url, repos_file_body, move |result| {
            let result = PJReposFileManager::update_repos_file_with_result(result);
            PJHttpReposFileRequest::dispatch_file_response(i_delegate, result, "PJ_CreateFile");
        }); 
    });
}

#[no_mangle]
pub unsafe extern "C" fn PJ_UpdateReposFile(
    delegate: IPJToDoHttpRequestDelegate,
    request_url: *const c_char,
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
    let request_url = CStr::from_ptr(request_url).to_string_lossy().into_owned();
    let repos_file_body = ReposFileBody::new(path, message, content, sha);

    thread::spawn(move || {
        PJHttpReposFileRequest::update_repos_file(request_url, repos_file_body, move |result| {
            let result = PJReposFileManager::update_repos_file_with_result(result);
            PJHttpReposFileRequest::dispatch_file_response(i_delegate, result, "PJ_UpdateFile");
        });
    });
}

#[no_mangle]
pub unsafe extern "C" fn PJ_DeleteReposFile(
    delegate: IPJToDoHttpRequestDelegate,
    request_url: *const c_char,
    path: *const c_char,
    message: *const c_char,
    content: *const c_char,
    sha: *const c_char,
) {
    if path == std::ptr::null_mut() || message == std::ptr::null_mut() || content == std::ptr::null_mut() || sha == std::ptr::null_mut() {
        pj_error!("path or message or content or sha: *mut PJ_DeleteFile is null!");
        assert!(path != std::ptr::null_mut() && message != std::ptr::null_mut() && content != std::ptr::null_mut() && sha != std::ptr::null_mut());
    }

    let i_delegate = IPJToDoHttpRequestDelegateWrapper(delegate);
    let request_url = CStr::from_ptr(request_url).to_string_lossy().into_owned();
    let repos_file_body = ReposFileBody::new(path, message, content, sha);

    thread::spawn(move || {
        PJHttpReposFileRequest::delete_repos_file(request_url, repos_file_body, move |result| {
            PJReposFileManager::remove_repos_file();
            PJHttpReposFileRequest::dispatch_file_response(i_delegate, result, "PJ_DeleteFile");
        });
    });
}

#[no_mangle]
pub unsafe extern "C" fn PJ_GetReposFile(
    delegate: IPJToDoHttpRequestDelegate,
    request_url: *const c_char,
) {
    if request_url == std::ptr::null_mut() {
        pj_error!("request_url: *mut PJ_GetFile is null!");
        assert!(request_url != std::ptr::null_mut());
    }

    let i_delegate = IPJToDoHttpRequestDelegateWrapper(delegate);
    let request_url = CStr::from_ptr(request_url).to_string_lossy().into_owned();

    thread::spawn(move || {
        PJHttpReposFileRequest::get_repos_file(request_url, move |result| {
            let result = PJReposFileManager::update_repos_file_with_result(result);
            PJHttpReposFileRequest::dispatch_file_response(i_delegate, result, "PJ_GetFile");
        });
    });
}
