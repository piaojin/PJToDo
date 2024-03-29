use crate::delegates::to_do_http_request_delegate::{
    IPJToDoHttpRequestDelegateWrapper, IPJToDoHttpRequestDelegate,
};
use std::ffi::CStr;
use crate::common::manager::pj_user_manager::PJUserManager;
use crate::network::http_request::PJHttpRequest;
use std::thread;
use crate::network::http_user_request::PJHttpUserRequest;
use libc::c_char;

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
pub unsafe extern "C" fn pj_login_via_access_token(delegate: IPJToDoHttpRequestDelegate) {
    let i_delegate = IPJToDoHttpRequestDelegateWrapper(delegate);

    thread::spawn(move || {
        PJHttpUserRequest::login_via_access_token(move |result| {
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
pub unsafe extern "C" fn pj_access_token(
    delegate: IPJToDoHttpRequestDelegate,
    code: *const c_char,
    client_id: *const c_char,
    client_secret: *const c_char,
) {
    if code == std::ptr::null_mut()
        || client_id == std::ptr::null_mut()
        || client_secret == std::ptr::null_mut()
    {
        pj_error!("params: is null!");
        assert!(
            code != std::ptr::null_mut()
                && client_id != std::ptr::null_mut()
                && client_secret != std::ptr::null_mut()
        );
    }

    let i_delegate = IPJToDoHttpRequestDelegateWrapper(delegate);
    let code = CStr::from_ptr(code).to_string_lossy().into_owned();
    let client_id = CStr::from_ptr(client_id).to_string_lossy().into_owned();
    let client_secret = CStr::from_ptr(client_secret).to_string_lossy().into_owned();

    thread::spawn(move || {
        PJHttpUserRequest::access_token(&code, &client_id, &client_secret, move |result| {
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
