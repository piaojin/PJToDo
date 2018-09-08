// {
//     "id": 216379469,
//     "url": "https://api.github.com/authorizations/216379469",
//     "app": {
//         "name": "create repo with ajax",
//         "url": "https://developer.github.com/v3/oauth_authorizations/",
//         "client_id": "00000000000000000000"
//     },
//     "token": "fa6b6ce6f8e2dba6246070acd3b1c791de840b68",
//     "hashed_token": "0fdaa0b141ae6cfe2224bb320842f72e3916b2cff1379bdecbafa88305b26a84",
//     "token_last_eight": "de840b68",
//     "note": "create repo with ajax",
//     "note_url": null,
//     "created_at": "2018-09-04T00:56:38Z",
//     "updated_at": "2018-09-04T00:56:38Z",
//     "scopes": [
//         "repo",
//         "admin:org",
//         "admin:public_key",
//         "admin:repo_hook",
//         "admin:org_hook",
//         "gist",
//         "notifications",
//         "user",
//         "delete_repo",
//         "write:discussion",
//         "admin:gpg_key"
//     ],
//     "fingerprint": null
// }

// { id: 216821089, url: "https://api.github.com/authorizations/216821089", token: "3893344ec9e010b856f1fc20753a50b070c83fbf", hashed_token: "832c003b47e7f0e8bc807dc567b39a3e46b541759c36a0dde95be72b8f4c0b37", created_at: "2018-09-05T09:48:35Z", updated_at: "2018-09-05T09:48:35Z" }
// TRACE - flushed({role=client}): State { reading: Closed, writing: Closed, keep_alive: Disabled, error: None }

extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use common::pj_serialize::PJSerdeDeserialize;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Authorizations<'a> {
    id: i64,
    url: &'a str,
    token: &'a str,
    hashed_token: &'a str,
    created_at: &'a str,
    updated_at: &'a str
}

impl<'a, 'b: 'a> PJSerdeDeserialize<'b> for Authorizations<'a> {
    type Item = Authorizations<'a>;
    fn new() -> Self::Item {
        Self::Item::default()
    }
}