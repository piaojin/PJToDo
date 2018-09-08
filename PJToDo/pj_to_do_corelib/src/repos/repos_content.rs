// {
//     "content": {
//         "name": "test4.md",
//         "path": "examples/test4.md",
//         "sha": "fa43f290c3e94a9940a7473c17c17980b0bd4b73",
//         "size": 27,
//         "url": "https://api.github.com/repos/piaojin/PJToDo/contents/examples/test4.md?ref=master",
//         "html_url": "https://github.com/piaojin/PJToDo/blob/master/examples/test4.md",
//         "git_url": "https://api.github.com/repos/piaojin/PJToDo/git/blobs/fa43f290c3e94a9940a7473c17c17980b0bd4b73",
//         "download_url": "https://raw.githubusercontent.com/piaojin/PJToDo/master/examples/test4.md",
//         "type": "file",
//         "_links": {
//             "self": "https://api.github.com/repos/piaojin/PJToDo/contents/examples/test4.md?ref=master",
//             "git": "https://api.github.com/repos/piaojin/PJToDo/git/blobs/fa43f290c3e94a9940a7473c17c17980b0bd4b73",
//             "html": "https://github.com/piaojin/PJToDo/blob/master/examples/test4.md"
//         }
//     },
//     "commit": {
//         "sha": "82f4e435ceccc667398bf635f764809b6f07b601",
//         "node_id": "MDY6Q29tbWl0MTQzNTk3NDE5OjgyZjRlNDM1Y2VjY2M2NjczOThiZjYzNWY3NjQ4MDliNmYwN2I2MDE=",
//         "url": "https://api.github.com/repos/piaojin/PJToDo/git/commits/82f4e435ceccc667398bf635f764809b6f07b601",
//         "html_url": "https://github.com/piaojin/PJToDo/commit/82f4e435ceccc667398bf635f764809b6f07b601",
//         "author": {
//             "name": "飘金",
//             "email": "13666902838@163.com",
//             "date": "2018-08-14T08:37:55Z"
//         },
//         "committer": {
//             "name": "飘金",
//             "email": "13666902838@163.com",
//             "date": "2018-08-14T08:37:55Z"
//         },
//         "tree": {
//             "sha": "98ff5fe155aa3d18453a2748a94bf794132a0286",
//             "url": "https://api.github.com/repos/piaojin/PJToDo/git/trees/98ff5fe155aa3d18453a2748a94bf794132a0286"
//         },
//         "message": "测试添加一个文件",
//         "parents": [
//             {
//                 "sha": "1013a903932322b5b980f081b35c47caa97f6f03",
//                 "url": "https://api.github.com/repos/piaojin/PJToDo/git/commits/1013a903932322b5b980f081b35c47caa97f6f03",
//                 "html_url": "https://github.com/piaojin/PJToDo/commit/1013a903932322b5b980f081b35c47caa97f6f03"
//             }
//         ],
//         "verification": {
//             "verified": false,
//             "reason": "unsigned",
//             "signature": null,
//             "payload": null
//         }
//     }
// }

extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use common::pj_serialize::PJSerdeDeserialize;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ReposFile<'a> {
    #[serde(borrow)]
    pub content: ReposContent<'a>
}

impl<'b: 'a, 'a> PJSerdeDeserialize<'b> for ReposFile<'a> {
    type Item = ReposFile<'a>;
    fn new() -> Self::Item {
        Self::Item::default()
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ReposContent<'a> {
    name: &'a str,
    path: &'a str,
    sha: &'a str,
    size: i64,
    url: &'a str,
    html_url: &'a str,
    git_url: &'a str,
    download_url: &'a str,
    #[serde(rename = "type")]
    _type: &'a str
}

impl<'a, 'b: 'a> PJSerdeDeserialize<'b> for ReposContent<'a> {
    type Item = ReposContent<'a>;
    fn new() -> Self::Item {
        Self::Item::default()
    }
}