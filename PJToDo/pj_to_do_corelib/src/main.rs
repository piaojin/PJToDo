#![feature(extern_prelude)]
#![feature(unboxed_closures)]
#![feature(custom_attribute)]
#![allow(proc_macro_derive_resolution_fallback)]
#[macro_use]
extern crate log;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate futures;

#[macro_use]
extern crate diesel;

pub mod to_do_type;
use to_do_type::to_do_type::{ToDoTypeForm, ToDoType};

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

#[macro_use]
pub mod db;
use db::pj_db_connection_util::PJDBConnectionUtil;
use db::tables::schema;
use diesel::prelude::*;

pub mod pal;

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

    // let repos_delete_file_body = ReposFileBody {
    //     path: "examples/test0.md",
    //     message: "测试添加一个文件",
    //     content: "aGVsbG8gd29ybGQsIGhlbGxvIHBpYW9qaW4h",
    //     sha: "fa43f290c3e94a9940a7473c17c17980b0bd4b73"
    // };

    // PJHttpReposRequest::delete_file(repos_delete_file_body, |result| {
    //     match result {
    //         Ok(repos_content) => {
    //             pj_info!("repos_content: {:?}", repos_content);
    //         },
    //         Err(e) => {
    //             pj_error!("request error: {:?}", e);
    //         }
    //     }
    // });

    // /Users/zoey.weng/Desktop/Study/PJToDo/ToDo.db
    let connection_util = PJDBConnectionUtil::new();

    // let to_do_type = ToDoTypeForm {
    //     type_name: String::from("分类2")
    // };

    // diesel::insert_into(schema::todotype::table)
    //     .values(&to_do_type)
    //     .execute(&connection_util.connection)
    //     .expect("Error saving new ToDoType");

    use schema::todotype::dsl::*;

    // let to_do_type = todotype.select(type_name.eq("分类2".to_string())).load::<ToDoType>(&connection_util.connection).unwrap();

    //查询所有数据，不带条件
    // let to_do_types = todotype.load::<ToDoType>(&connection_util.connection).unwrap();
    // pj_info!("to_do_type: {:?}", to_do_types);

    // //查找所有type_name="分类2"的数据集合
    // let to_do_types2 = schema::todotype::table.filter(type_name.eq("分类2".to_string())).load::<ToDoType>(&connection_util.connection).unwrap();
    // pj_info!("to_do_type2: {:?}", to_do_types2);

    // //取出结果中的第一条记录
    // let to_do_types2_first = to_do_types2.first();
    // pj_info!("to_do_types2_first: {:?}", to_do_types2_first);

    //通过ID查找
    // let t = todotype.find(1).first::<ToDoType>(&connection_util.connection);
    // pj_info!("t: {:?}", t);

    //更新数据通过传入更新字段
    // use diesel::{update};
    // update(todotype.filter(id.eq(1)))
    //     .set(type_name.eq("Jim"))
    //     .execute(&connection_util.connection)
    //     .unwrap();

    //更新数据通过传入struct
    // let newType = ToDoTypeForm {
    //     type_name: String::from("hello piaojin!")
    // };
    // use diesel::{update};
    // update(todotype.filter(id.eq(1)))
    //     .set(&newType)
    //     .execute(&connection_util.connection)
    //     .unwrap();

    //删除数据
    // use diesel::{delete};
    // let deleted_rows = delete(todotype.filter(type_name.eq("hello piaojin!"))).execute(&connection_util.connection);
    // pj_info!("deleted_rows: {:?}", deleted_rows);
}

// impl Queryable<users::SqlType, DB> for User {
//     type Row = (i32, String);

//     fn build(row: Self::Row) -> Self {
//         User {
//             id: row.0,
//             name: row.1.to_lowercase(),
//         }
//     }
// }