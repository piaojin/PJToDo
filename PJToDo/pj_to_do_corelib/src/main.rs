#![feature(extern_prelude)]
#![feature(unboxed_closures)]
// #![feature(use_extern_macros)]
#[macro_use]
extern crate log;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate futures;

pub mod to_do_type;
use to_do_type::to_do_type::ToDoType;

#[macro_use]
pub mod common;
use common::pj_logger::PJLogger;

pub mod network;
use network::http_user_request::PJHttpUserRequest;
use network::http_repos_request::PJHttpReposRequest;
use common::request_config::PJRequestConfig;

pub mod mine;

pub mod repos;
use repos::repos_file::ReposFileBody;

fn main() {
    //使用log之前需要初始化，并且只需要初始化一次
    let _ = PJLogger::pj_init_logger();

    // let to_do_type = ToDoType {
    //     id: 10,
    //     type_name: "分类"
    // };
    // pj_info!("to_do_type: {:?}", to_do_type);

    // let data = PJHttpRequest::fetch_response_data();
    // let result = data
    //     .map(|body: hyper::Chunk| {
    //         let user: User = PJHttpRequest::parse_data::<User>(&body);
    //         pj_info!("user: {:?}", user);
    //     })
    //     // if there was an error print it
    //     .map_err(|e| {
    //         eprintln!("json parsing error: {:?}", e)
    //     });
    // hyper::rt::run(result);

    // PJHttpUserRequest::request_user_info(|result| {
    //     match result {
    //         Ok(user) => {
    //             pj_info!("user: {:?}", user);
    //         },
    //         Err(e) => {
    //             pj_error!("request error: {:?}", e);
    //         }
    //     }
    // });

    // PJHttpUserRequest::login("piaojin", "weng804488815", |result| {
    //     match result {
    //         Ok(user) => {
    //             pj_info!("user: {:?}", user);
    //         },
    //         Err(e) => {
    //             pj_error!("request error: {:?}", e);
    //         }
    //     }
    // });

    // PJHttpUserRequest::authorizations("Basic cGlhb2ppbjp3ZW5nODA0NDg4ODE1", |result| {
    //     match result {
    //         Ok(authorization) => {
    //             pj_info!("authorization: {:?}", authorization);
    //         },
    //         Err(e) => {
    //             pj_error!("request error: {:?}", e);
    //         }
    //     }
    // });

    // PJHttpReposRequest::create_repos(PJRequestConfig::repos_request_body(), |result| {
    //     match result {
    //         Ok(repos) => {
    //             pj_info!("repos: {:?}", repos);
    //         },
    //         Err(e) => {
    //             pj_error!("request error: {:?}", e);
    //         }
    //     }
    // });

    // PJHttpReposRequest::get_repos("https://api.github.com/repos/piaojin/Hello-World", |result| {
    //     match result {
    //         Ok(repos) => {
    //             pj_info!("repos: {:?}", repos);
    //         },
    //         Err(e) => {
    //             pj_error!("request error: {:?}", e);
    //         }
    //     }
    // });

    // PJHttpReposRequest::delete_repos("https://api.github.com/repos/piaojin/Hello-World", |result| {
    //     match result {
    //         Ok(repos) => {
    //             pj_info!("repos: {:?}", repos);
    //         },
    //         Err(e) => {
    //             pj_error!("request error: {:?}", e);
    //         }
    //     }
    // });

    // let repos_create_file_body = ReposFileBody {
    //     path: "examples/test0.md",
    //     message: "测试添加一个文件",
    //     content: "aGVsbG8gd29ybGQsIGhlbGxvIHBpYW9qaW4haGVsbG8gd29ybGQsIGhlbGxvIHBpYW9qaW4h",
    //     sha: ""
    // };

    // PJHttpReposRequest::create_file(repos_create_file_body, |result| {
    //     match result {
    //         Ok(repos_content) => {
    //             pj_info!("repos_content: {:?}", repos_content);
    //         },
    //         Err(e) => {
    //             pj_error!("request error: {:?}", e);
    //         }
    //     }
    // });

    // let repos_update_file_body = ReposFileBody {
    //     path: "examples/test0.md",
    //     message: "测试添加一个文件",
    //     content: "aGVsbG8gd29ybGQsIGhlbGxvIHBpYW9qaW4h",
    //     sha: "c614da0a3e9b448f8d779e409307217ea4f0ed3d"
    // };

    // PJHttpReposRequest::update_file(repos_update_file_body, |result| {
    //     match result {
    //         Ok(repos_content) => {
    //             pj_info!("repos_content: {:?}", repos_content);
    //         },
    //         Err(e) => {
    //             pj_error!("request error: {:?}", e);
    //         }
    //     }
    // });

    let repos_delete_file_body = ReposFileBody {
        path: "examples/test0.md",
        message: "测试添加一个文件",
        content: "aGVsbG8gd29ybGQsIGhlbGxvIHBpYW9qaW4h",
        sha: "fa43f290c3e94a9940a7473c17c17980b0bd4b73"
    };

    PJHttpReposRequest::delete_file(repos_delete_file_body, |result| {
        match result {
            Ok(repos_content) => {
                pj_info!("repos_content: {:?}", repos_content);
            },
            Err(e) => {
                pj_error!("request error: {:?}", e);
            }
        }
    });
}

