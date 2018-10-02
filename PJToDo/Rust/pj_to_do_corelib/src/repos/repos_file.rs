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

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ReposFileBody<'a> {
    pub path: &'a str,
    pub message: &'a str,
    pub content: &'a str,
    pub sha: &'a str
}

impl<'a, 'b: 'a> PJSerdeDeserialize<'b> for ReposFileBody<'a> {
    type Item = ReposFileBody<'a>;
    fn new() -> Self::Item {
        Self::Item::default()
    }
}