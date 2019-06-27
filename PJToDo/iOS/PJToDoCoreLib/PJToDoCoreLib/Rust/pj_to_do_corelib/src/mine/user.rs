// {
//     "login": "piaojin",
//     "id": 10163672,
//     "node_id": "MDQ6VXNlcjEwMTYzNjcy",
//     "avatar_url": "https://avatars2.githubusercontent.com/u/10163672?v=4",
//     "gravatar_id": "",
//     "url": "https://api.github.com/users/piaojin",
//     "html_url": "https://github.com/piaojin",
//     "followers_url": "https://api.github.com/users/piaojin/followers",
//     "following_url": "https://api.github.com/users/piaojin/following{/other_user}",
//     "gists_url": "https://api.github.com/users/piaojin/gists{/gist_id}",
//     "starred_url": "https://api.github.com/users/piaojin/starred{/owner}{/repo}",
//     "subscriptions_url": "https://api.github.com/users/piaojin/subscriptions",
//     "organizations_url": "https://api.github.com/users/piaojin/orgs",
//     "repos_url": "https://api.github.com/users/piaojin/repos",
//     "events_url": "https://api.github.com/users/piaojin/events{/privacy}",
//     "received_events_url": "https://api.github.com/users/piaojin/received_events",
//     "type": "User",
//     "site_admin": false,
//     "name": "飘金",
//     "company": "https://piaojin.github.io/",
//     "blog": "http://www.jianshu.com/u/da7864faa1be",
//     "location": null,
//     "email": null,
//     "hireable": true,
//     "bio": "never say goodbye!",
//     "public_repos": 49,
//     "public_gists": 0,
//     "followers": 7,
//     "following": 0,
//     "created_at": "2014-12-12T07:57:57Z",
//     "updated_at": "2018-08-12T13:02:19Z",
//     "private_gists": 0,
//     "total_private_repos": 0,
//     "owned_private_repos": 0,
//     "disk_usage": 306783,
//     "collaborators": 0,
//     "two_factor_authentication": false,
//     "plan": {
//         "name": "free",
//         "space": 976562499,
//         "collaborators": 0,
//         "private_repos": 0
//     }
// }

extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use common::pj_serialize::PJSerdeDeserialize;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct User {
    pub login: String,
    pub id: i64,
    pub node_id: String,
    pub avatar_url: String,
    pub gravatar_id: String,
    pub url: String,
    pub html_url: String,
    pub followers_url: String,
    pub following_url: String,
    pub gists_url: String,
    pub starred_url: String,
    pub subscriptions_url: String,
    pub organizations_url: String,
    pub repos_url: String,
    pub events_url: String,
    pub received_events_url: String,
    //给json中对应的字段重新命名
    #[serde(rename = "type")]
    pub _type: String,
    pub site_admin: bool,
    pub name: String,
    pub company: String,
    pub blog: String,
    pub bio: String,
    pub public_repos: i64,
    pub public_gists: i64,
    pub followers: i64,
    pub following: i64,
    pub created_at: String,
    pub updated_at: String,
}

// impl User {
//     pub fn new() -> User {
//         User {
//             login: "".to_string(),
//             id: 0,
//             node_id: "".to_string(),
//             avatar_url: "".to_string(),
//             gravatar_id: "".to_string(),
//             url: "".to_string(),
//             html_url: "".to_string(),
//             followers_url: "".to_string(),
//             following_url: "".to_string(),
//             gists_url: "".to_string(),
//             starred_url: "".to_string(),
//             subscriptions_url: "".to_string(),
//             organizations_url: "".to_string(),
//             repos_url: "".to_string(),
//             events_url: "".to_string(),
//             received_events_url: "".to_string(),
//             _type: "".to_string(),
//             site_admin: false,
//             name: "".to_string(),
//             company: "".to_string(),
//             blog: "".to_string(),
//             bio: "".to_string(),
//             public_repos: 0,
//             public_gists: 0,
//             followers: 0,
//             following: 0,
//             created_at: "".to_string(),
//             updated_at: "".to_string(),
//         }
//     }
// }

impl<'b> PJSerdeDeserialize<'b> for User {
    type Item = User;
    fn new() -> Self::Item {
        Self::Item::default()
    }
}
