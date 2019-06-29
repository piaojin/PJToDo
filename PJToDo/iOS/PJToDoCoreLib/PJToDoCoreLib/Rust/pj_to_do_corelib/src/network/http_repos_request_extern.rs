use network::http_repos_request::PJHttpReposRequest;
use delegates::to_do_http_request_delegate::{
    IPJToDoHttpRequestDelegateWrapper, IPJToDoHttpRequestDelegate,
};
use std::ffi::{CStr};
use libc::{c_char};
use common::request_config::PJRequestConfig;
use std::thread;
use common::manager::pj_repos_manager::PJReposManager;

#[no_mangle]
pub unsafe extern "C" fn pj_create_repos(delegate: IPJToDoHttpRequestDelegate) {
    let i_delegate = IPJToDoHttpRequestDelegateWrapper(delegate);

    thread::spawn(move || {
        PJHttpReposRequest::create_repos(PJRequestConfig::repos_request_body(), move |result| {
            let result = PJReposManager::update_repos_with_result(result);
            PJHttpReposRequest::dispatch_repos_response(i_delegate, result, "PJ_CreateRepos");
        });
    });
}

#[no_mangle]
pub unsafe extern "C" fn pj_get_repos(
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
            let result = PJReposManager::update_repos_with_result(result);
            PJHttpReposRequest::dispatch_repos_response(i_delegate, result, "PJ_GetRepos");
        });
    });
}

#[no_mangle]
pub unsafe extern "C" fn pj_delete_repos(
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
