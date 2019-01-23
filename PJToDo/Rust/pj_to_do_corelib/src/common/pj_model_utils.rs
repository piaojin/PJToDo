use std::sync::Mutex;
use mine::user::User;

lazy_static! {
    pub static ref USERINFO: Mutex<User> = Mutex::new(User::new());
}

pub struct PJModelUtils;

impl PJModelUtils {
    pub fn update_user(user: User) {
        *(USERINFO.lock().unwrap()) = user;
    }
}