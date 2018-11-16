// {
// 	"path":"examples/test4.md",
//     "message":"测试更新一个文件",
//     "content":"5rWL6K+V5pu05paw5LiA5Liq5paH5Lu2",
//     "sha": "fa43f290c3e94a9940a7473c17c17980b0bd4b73"
// }

extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use common::pj_serialize::PJSerdeDeserialize;
use std::ffi::{CStr};
use libc::{c_char};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ReposFileBody {
    pub path: String,
    pub message: String,
    pub content: String,
    pub sha: String,
}

impl ReposFileBody {
    pub unsafe fn new(
        path: *const c_char,
        message: *const c_char,
        content: *const c_char,
        sha: *const c_char,
    ) -> ReposFileBody {
        let path = CStr::from_ptr(path).to_string_lossy().into_owned();
        let message = CStr::from_ptr(message).to_string_lossy().into_owned();
        let content = CStr::from_ptr(content).to_string_lossy().into_owned();
        let sha = CStr::from_ptr(sha).to_string_lossy().into_owned();

        let repos_file_body = ReposFileBody {
            path: path,
            message: message,
            content: content,
            sha: sha,
        };

        repos_file_body
    }
}

impl<'b> PJSerdeDeserialize<'b> for ReposFileBody {
    type Item = ReposFileBody;
    fn new() -> Self::Item {
        Self::Item::default()
    }
}
