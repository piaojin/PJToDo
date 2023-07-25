use repos::repos::ReposRequestBody;

pub struct PJRequestConfig;

//user
impl PJRequestConfig {
    //get user info
    pub fn user_info<'a>() -> &'a str {
        "https://api.github.com/users/piaojin"
    }

    //login
    pub fn login<'a>() -> &'a str {
        "https://api.github.com/user"
    }
}

//authorization
impl PJRequestConfig {
    pub fn authorization_head<'a>() -> &'a str {
        "Authorization"
    }

    //authorizations
    pub fn authorizations<'a>() -> &'a str {
        "https://api.github.com/authorizations"
    }

    //authorizations body
    pub fn authorization_body() -> &'static str {
        r#"{
            "scopes":["repo","admin:org","admin:public_key","admin:repo_hook","admin:org_hook","gist","notifications","user","delete_repo","write:discussion","admin:gpg_key"],
            "note":"create authorization, piaojin todo!"
            }"#
    }

    //personal token
    pub fn personal_token() -> &'static str {
        //from pal
        "token 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"
    }
}

//repos
impl PJRequestConfig {
    //create repos
    pub fn repos<'a>() -> &'a str {
        "https://api.github.com/user/repos"
    }

    pub fn repos_request_body<'a>() -> ReposRequestBody<'a> {
        ReposRequestBody {
            name: "PJToDoWebDataBase",
            description: "This is your PJToDoWebDataBase repository",
            homepage: "https://github.com",
            private: false,
            has_issues: true,
            has_projects: true,
            has_wiki: true,
        }
    }

    pub fn get_repos_contents_url<'a>() -> &'a str {
        //the url will get from pal
        "https://api.github.com/repos/piaojin/Hello-World/contents/"
    }
}
