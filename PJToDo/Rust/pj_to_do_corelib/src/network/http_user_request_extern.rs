use delegates::to_do_http_request_delegate::{IPJToDoHttpRequestDelegateWrapper, IPJToDoHttpRequestDelegate};
use std::ffi::{CStr};
use common::manager::pj_user_manager::PJUserManager;
use network::http_request::{PJHttpRequest};
use std::thread;
use network::http_user_request::PJHttpUserRequest;
use libc::{c_char};

#[no_mangle]
pub unsafe extern "C" fn pj_login(
    delegate: IPJToDoHttpRequestDelegate,
    name: *const c_char,
    password: *const c_char,
) {
    if name == std::ptr::null_mut() || password == std::ptr::null_mut() {
        pj_error!("name or password: *mut PJ_Login is null!");
        assert!(name != std::ptr::null_mut() && password != std::ptr::null_mut());
    }

    let i_delegate = IPJToDoHttpRequestDelegateWrapper(delegate);
    let name = CStr::from_ptr(name).to_string_lossy().into_owned();
    let password = CStr::from_ptr(password).to_string_lossy().into_owned();

    thread::spawn(move || {
        PJHttpUserRequest::login(&name, &password, move |result| {
            let result = PJUserManager::update_user_with_result(result);
            PJHttpRequest::dispatch_http_response(result, i_delegate);
        });
    });
}

#[no_mangle]
pub unsafe extern "C" fn pj_authorizations(
    delegate: IPJToDoHttpRequestDelegate,
    authorization: *const c_char,
) {
    if authorization == std::ptr::null_mut() {
        pj_error!("authorization: *mut PJ_Authorizations is null!");
        assert!(authorization != std::ptr::null_mut());
    }

    let i_delegate = IPJToDoHttpRequestDelegateWrapper(delegate);
    let authorization = CStr::from_ptr(authorization).to_string_lossy().into_owned();

    thread::spawn(move || {
        PJHttpUserRequest::authorizations(&authorization, move |result| {
            PJHttpRequest::dispatch_http_response(result, i_delegate);
        });
    });
}

#[no_mangle]
pub unsafe extern "C" fn pj_request_user_info(delegate: IPJToDoHttpRequestDelegate) {
    let i_delegate = IPJToDoHttpRequestDelegateWrapper(delegate);

    thread::spawn(move || {
        PJHttpUserRequest::request_user_info(move |result| {
            PJHttpRequest::dispatch_http_response(result, i_delegate);
        });
    });
}

#[no_mangle]
pub unsafe extern "C" fn pj_logout() {
    PJUserManager::logout();
}
