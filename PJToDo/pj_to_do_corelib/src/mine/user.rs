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
pub struct User<'a> {
    pub login: &'a str,
    pub id: i64,
    pub node_id: &'a str,
    pub avatar_url: &'a str,
    pub gravatar_id: &'a str,
    pub url: &'a str,
    pub html_url: &'a str,
    pub followers_url: &'a str,
    pub following_url: &'a str,
    pub gists_url: &'a str,
    pub starred_url: &'a str,
    pub subscriptions_url: &'a str,
    pub organizations_url: &'a str,
    pub repos_url: &'a str,
    pub events_url: &'a str,
    pub received_events_url: &'a str,
    //给json中对应的字段重新命名
    #[serde(rename = "type")]
    pub _type: &'a str,
    pub site_admin: bool,
    pub name: &'a str,
    pub company: &'a str,
    pub blog: &'a str,
    // #[serde(default = "no location")]
    // location: &'a str,
    // #[serde(default = "no email")]
    // email: &'a str,
    pub hireable: bool,
    pub bio: &'a str,
    pub public_repos: i64,
    pub public_gists: i64,
    pub followers: i64,
    pub following: i64,
    pub created_at: &'a str,
    pub updated_at: &'a str,
    // private_gists: i64,
    // total_private_repos: i64,
    // owned_private_repos: i64,
    // disk_usage: i64,
    // collaborators: i64,
    // two_factor_authentication: bool,
    // plan: Plan
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Plan {
    name: String,
    space: i64,
    collaborators: i64,
    private_repos: i64
}

impl<'a, 'b: 'a> PJSerdeDeserialize<'b> for User<'a> {
    type Item = User<'a>;
    fn new() -> Self::Item {
        Self::Item::default()
    }
}