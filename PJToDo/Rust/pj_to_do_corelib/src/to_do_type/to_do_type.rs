extern crate serde_derive;
extern crate serde;
extern crate serde_json;

extern crate libc;
use self::libc::{c_char};
use std::ffi::CStr;

// use diesel::associations::BelongsTo;
// use diesel::*;

// use diesel::prelude::*;
// use diesel::backend::Backend;
// use diesel::types::{FromSqlRow, HasSqlType};
// use diesel::row::Row;
// use std::error::Error;

use db::tables::schema::{todotype};
// #[primary_key(id)]
// #[column_name(barId)]
#[repr(C)]
#[derive(
    Serialize, Deserialize, Debug, Default, PartialEq, Queryable, AsChangeset, Identifiable,
)]
#[table_name = "todotype"]
pub struct ToDoType {
    pub id: i32,
    pub type_name: String,
}

//如果ToDo中不包含生命周期的属性则可以使用#[derive(Associations)]替代一下代码
//Unreleased:
//#[belongs_to] can now accept types that are generic over lifetimes (for example, if one of the fields has the type Cow<'a, str>). To define an association to such a type, write #[belongs_to(parent = "User<'_>")]
// impl<'a> BelongsTo<ToDo<'a>> for ToDoType {
//     type ForeignKey = i32;
//     type ForeignKeyColumn = todotype::to_do_id;

//     fn foreign_key(&self) -> Option<&Self::ForeignKey> {
//         Some(&self.to_do_id)
//     }

//     fn foreign_key_column() -> Self::ForeignKeyColumn {
//         todotype::to_do_id
//     }
// }

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

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Insertable)]
#[table_name = "todotype"]
pub struct ToDoTypeInsert {
    pub type_name: String,
}

impl ToDoTypeInsert {
    pub fn new(type_name: String) -> ToDoTypeInsert {
        ToDoTypeInsert {
            type_name: type_name,
        }
    }
}

/*** extern "C" ***/

#[no_mangle]
pub unsafe extern "C" fn createToDoTypeInsert(type_name: *const c_char) -> *mut ToDoTypeInsert {
    let type_name = CStr::from_ptr(type_name).to_string_lossy().into_owned(); //unsafe
    Box::into_raw(Box::new(ToDoTypeInsert::new(type_name)))
}
