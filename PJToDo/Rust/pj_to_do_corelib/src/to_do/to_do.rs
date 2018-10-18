extern crate serde_derive;
extern crate serde;
extern crate serde_json;

// use diesel::associations::BelongsTo;
use common::pj_serialize::PJSerdeDeserialize;
use to_do_tag::to_do_tag::ToDoTag;
use to_do_type::to_do_type::ToDoType;
// use std::io::Write;
// use diesel::sqlite::Sqlite;
// use diesel::serialize::{self, IsNull, Output, ToSql};
// use diesel::deserialize::{self, FromSql};
// use diesel::sqlite::connection::sqlite_value::SqliteValue;

use db::tables::schema::todo;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum ToDoState {
    Determined,
    InProgress,
    Finished,
    Overdue,
}

// impl ToSql<ToDoStateType, Sqlite> for ToDoState {
//     fn to_sql<W: Write>(&self, out: &mut Output<W, Sqlite>) -> serialize::Result {
//         match *self {
//             ToDoState::Determined => out.write_all(b"determined")?,
//             ToDoState::InProgress => out.write_all(b"inprogress")?,
//             ToDoState::Finished => out.write_all(b"finished")?,
//             ToDoState::Overdue => out.write_all(b"overdue")?,
//         }
//         Ok(IsNull::No)
//     }
// }

// impl FromSql<ToDoStateType, Sqlite> for ToDoState {
//     fn from_sql(bytes: Option<&SqliteValue>) -> deserialize::Result<Self> {
//         match not_none!(bytes) {
//             b"determined" => Ok(ToDoState::Determined),
//             b"inprogress" => Ok(ToDoState::InProgress),
//             b"finished" => Ok(ToDoState::Finished),
//             b"overdue" => Ok(ToDoState::Overdue),
//             _ => Err("Unrecognized enum variant".into()),
//         }
//     }
// }

impl Default for ToDoState {
    fn default() -> ToDoState {
        ToDoState::Determined
    }
}

#[derive(
    Serialize, Deserialize, Debug, Default, PartialEq, Identifiable, Queryable, Associations,
)]
#[belongs_to(ToDoType, foreign_key = "to_do_type_id")]
#[belongs_to(ToDoTag, foreign_key = "to_do_tag_id")]
#[table_name = "todo"]
pub struct ToDoQuery {
    pub id: i32,
    pub content: String,     //待办事项内容
    pub title: String,       //待办事项标题
    pub due_time: String,    //到期时间
    pub remind_time: String, //提醒时间
    // #[sql_type="Text"]
    pub create_time: String, //创建时间
    pub update_time: String, //更新时间
    pub to_do_type_id: i32,  //标签
    pub to_do_tag_id: i32,   //分类
    pub state: i32,          //状态
}

// use diesel::sql_types::*;
// use diesel::expression::AsExpression;
// impl<'expr> AsExpression<Text> for &'expr str {

// }

//如果ToDo中不包含生命周期的属性则可以使用#[derive(Associations)]替代一下代码
//Unreleased:
//#[belongs_to] can now accept types that are generic over lifetimes (for example, if one of the fields has the type Cow<'a, str>). To define an association to such a type, write #[belongs_to(parent = "User<'_>")]
// impl<'a> BelongsTo<ToDoType> for ToDo<'a> {
//     type ForeignKey = i32;
//     type ForeignKeyColumn = todo::to_do_type_id;

//     fn foreign_key(&self) -> Option<&Self::ForeignKey> {
//         Some(&self.to_do_type_id)
//     }

//     fn foreign_key_column() -> Self::ForeignKeyColumn {
//         todo::to_do_type_id
//     }
// }

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Insertable)]
#[table_name = "todo"]
pub struct ToDoInsert {
    pub content: String,     //待办事项内容
    pub title: String,       //待办事项标题
    pub due_time: String,    //到期时间
    pub remind_time: String, //提醒时间
    pub create_time: String, //创建时间
    pub update_time: String, //更新时间
    pub to_do_type_id: i32,  //标签
    pub to_do_tag_id: i32,   //分类
    pub state: i32,          //状态
}

#[repr(C)]
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ToDo {
    pub id: i32,
    pub content: String,      //待办事项内容
    pub title: String,        //待办事项标题
    pub due_time: String,     //到期时间
    pub remind_time: String,  //提醒时间
    pub create_time: String,  //创建时间
    pub update_time: String,  //更新时间
    pub to_do_type_id: i32,   //标签
    pub to_do_tag_id: i32,    //分类
    pub to_do_tag: ToDoTag,   //标签
    pub to_do_type: ToDoType, //分类
    pub state: ToDoState,     //状态
    pub state_raw_value: i32, //状态
}

impl<'b> PJSerdeDeserialize<'b> for ToDo {
    type Item = ToDo;
    fn new() -> Self::Item {
        Self::Item::default()
    }
}

impl<'b> PJSerdeDeserialize<'b> for ToDoQuery {
    type Item = ToDoQuery;
    fn new() -> Self::Item {
        Self::Item::default()
    }
}

impl<'b> PJSerdeDeserialize<'b> for ToDoInsert {
    type Item = ToDoInsert;
    fn new() -> Self::Item {
        Self::Item::default()
    }
}
