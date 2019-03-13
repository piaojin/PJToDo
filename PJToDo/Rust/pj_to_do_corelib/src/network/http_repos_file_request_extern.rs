use network::http_repos_file_request::PJHttpReposFileRequest;
use delegates::to_do_http_request_delegate::{IPJToDoHttpRequestDelegateWrapper, IPJToDoHttpRequestDelegate};
use std::ffi::{CStr, CString};
use libc::{c_char};
use repos::repos_file::ReposFileBody;
use std::thread;
use common::manager::pj_repos_file_manager::PJReposFileManager;
use common::manager::pj_file_manager::PJFileManager;

#[no_mangle]
pub unsafe extern "C" fn pj_create_repos_file(
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
pub unsafe extern "C" fn pj_update_repos_file(
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
pub unsafe extern "C" fn pj_delete_repos_file(
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
pub unsafe extern "C" fn pj_get_repos_file(
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

#[no_mangle]
pub unsafe extern "C" fn pj_download_file(
    delegate: IPJToDoHttpRequestDelegate,
    request_url: *const c_char,
    save_path: *const c_char,
) {
    if save_path == std::ptr::null_mut() || request_url == std::ptr::null_mut() {
        pj_error!("path or message or content or sha: *mut PJ_DownLoadFile is null!");
        assert!(save_path != std::ptr::null_mut() && request_url != std::ptr::null_mut());
    }

    let i_delegate = IPJToDoHttpRequestDelegateWrapper(delegate);
    let request_url = CStr::from_ptr(request_url).to_string_lossy().into_owned();
    let save_path = CStr::from_ptr(save_path).to_string_lossy().into_owned();

    thread::spawn(move || {

        PJHttpReposFileRequest::download_file(request_url, move |result| {
            match result {
            Ok((status, body)) => {
                match PJFileManager::wirte_bytes_to_file(save_path, &body) {
                    Ok(_) => {
                        (i_delegate.request_result)(i_delegate.user, CString::new("".to_string()).unwrap().into_raw(), status.as_u16(), status.is_success());
                    },
                    Err(e) => {
                        pj_error!("download error: {:?}", e);
                        (i_delegate.request_result)(i_delegate.user, CString::new(format!("io error: {:?}", e)).unwrap().into_raw(), 0, false);
                    }
                }
            },
            Err(e) => {
                pj_error!("download error: {:?}", e);
                (i_delegate.request_result)(i_delegate.user, CString::new(format!("download error: {:?}", e)).unwrap().into_raw(), 0, false);
            }
        };
        });
    });
}
