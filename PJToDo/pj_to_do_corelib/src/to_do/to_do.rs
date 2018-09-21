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

#[derive(SqlType)]
#[sqlite(type_name = "todo_state_type")]
pub struct ToDoStateType;

#[derive(Serialize, Deserialize, Debug, FromSqlRow, AsExpression, PartialEq)]
#[sql_type = "ToDoStateType"]
pub enum ToDoState {
    Determined,
    InProgress,
    Finished,
    Overdue
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

#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
#[derive(Identifiable, Queryable, Associations)]
#[belongs_to(ToDoType, foreign_key = "to_do_type_id")]
#[table_name = "todo"]
pub struct ToDoQuery<'a> {
    pub id: i32,
    pub content: &'a str, //待办事项内容
    pub title: &'a str, //待办事项标题
    pub due_time: &'a str, //到期时间
    pub remind_time: &'a str, //提醒时间
    pub create_time: &'a str, //创建时间
    pub update_time: &'a str, //更新时间
    pub to_do_type_id: i32, //标签
    pub to_do_tag_id: i32, //分类
    pub state: ToDoState //状态
}

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

#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
#[derive(Insertable)]
#[table_name = "todo"]
pub struct ToDoInsert<'a> {
    pub content: &'a str, //待办事项内容
    pub title: &'a str, //待办事项标题
    pub due_time: &'a str, //到期时间
    pub remind_time: &'a str, //提醒时间
    pub create_time: &'a str, //创建时间
    pub update_time: &'a str, //更新时间
    pub to_do_type_id: i32, //标签
    pub to_do_tag_id: i32, //分类
    pub state: ToDoState //状态
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ToDo<'a> {
    pub id: i32,
    pub content: &'a str, //待办事项内容
    pub title: &'a str, //待办事项标题
    pub due_time: &'a str, //到期时间
    pub remind_time: &'a str, //提醒时间
    pub create_time: &'a str, //创建时间
    pub update_time: &'a str, //更新时间
    pub to_do_type_id: i32, //标签
    pub to_do_tag_id: i32, //分类
    pub to_do_tag: ToDoTag, //标签
    pub to_do_type: ToDoType, //分类
    pub state: ToDoState //状态
}

impl<'a, 'b: 'a> PJSerdeDeserialize<'b> for ToDo<'a> {
    type Item = ToDo<'a>;
    fn new() -> Self::Item {
        Self::Item::default()
    }
}

impl<'a, 'b: 'a> PJSerdeDeserialize<'b> for ToDoQuery<'a> {
    type Item = ToDoQuery<'a>;
    fn new() -> Self::Item {
        Self::Item::default()
    }
}

impl<'a, 'b: 'a> PJSerdeDeserialize<'b> for ToDoInsert<'a> {
    type Item = ToDoInsert<'a>;
    fn new() -> Self::Item {
        Self::Item::default()
    }
}