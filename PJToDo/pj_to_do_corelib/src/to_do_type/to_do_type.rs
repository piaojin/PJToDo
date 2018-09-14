extern crate serde_derive;
extern crate serde;
extern crate serde_json;

// use diesel::prelude::*;
// use diesel::backend::Backend;
// use diesel::types::{FromSqlRow, HasSqlType};
// use diesel::row::Row;
// use std::error::Error;

use db::tables::schema::todotype;
// #[primary_key(imei)]
// #[belongs_to(User)]
#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Queryable)]
pub struct ToDoType {
    pub id: i32,
    pub type_name: String,
}

// impl<ST, DB> FromSqlRow<ST, DB> for ToDoType where
//     DB: Backend + HasSqlType<ST>,
//     ToDoType: Queryable<ST, DB>,
//     <ToDoType as Queryable<ST, DB>>::Row: FromSqlRow<ST, DB>,
// {
//     fn build_from_row<T: Row<DB>>(row: &mut T) -> Result<Self, Box<Error + Send + Sync>> {
//         let row = try!(<<ToDoType as Queryable<ST, DB>>::Row as FromSqlRow<ST, DB>>::build_from_row(row));
//         Ok(ToDoType::build(row))
//     }
// }

//如果需要特殊处理一些字段
// impl Queryable<users::SqlType, DB> for ToDoType {
//     type Row = (i32, String);

//     fn build(row: Self::Row) -> Self {
//         User {
//             id: row.0,
//             name: row.1.to_lowercase(),
//         }
//     }
// }

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Insertable, AsChangeset)]
#[table_name = "todotype"]
pub struct ToDoTypeForm {
    // #[serde(rename = "id")]
    // pub id: i32,
    pub type_name: String,
}