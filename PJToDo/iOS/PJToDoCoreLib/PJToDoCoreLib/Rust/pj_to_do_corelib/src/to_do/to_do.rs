extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use common::pj_serialize::PJSerdeDeserialize;
use to_do_tag::to_do_tag::ToDoTag;
use to_do_type::to_do_type::ToDoType;

use db::tables::schema::todo;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum ToDoState {
    InProgress,
    UnDetermined,
    Completed,
    Overdue,
}

impl Default for ToDoState {
    fn default() -> ToDoState {
        ToDoState::UnDetermined
    }
}

#[derive(
    Serialize,
    Deserialize,
    Debug,
    Default,
    PartialEq,
    Identifiable,
    Queryable,
    Associations,
    AsChangeset,
    QueryableByName,
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
    pub priority: i32,
    pub to_do_type_id: i32, //标签
    pub to_do_tag_id: i32,  //分类
    pub state: i32,         //状态
}

impl ToDoQuery {
    pub fn from(todo_query: &ToDoQuery) -> ToDoQuery {
        ToDoQuery {
            id: todo_query.id,
            content: todo_query.content.clone(),
            title: todo_query.title.clone(),
            due_time: todo_query.due_time.clone(),
            remind_time: todo_query.remind_time.clone(),
            create_time: todo_query.create_time.clone(),
            update_time: todo_query.update_time.clone(),
            priority: todo_query.priority,
            to_do_type_id: todo_query.to_do_type_id,
            to_do_tag_id: todo_query.to_do_tag_id,
            state: todo_query.state,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Insertable)]
#[table_name = "todo"]
pub struct ToDoInsert {
    pub content: String,     //待办事项内容
    pub title: String,       //待办事项标题
    pub due_time: String,    //到期时间
    pub remind_time: String, //提醒时间
    pub create_time: String, //创建时间
    pub update_time: String, //更新时间
    pub priority: i32,
    pub to_do_type_id: i32, //标签
    pub to_do_tag_id: i32,  //分类
    pub state: i32,         //状态
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

/*** extern "C" ***/

#[no_mangle]
pub unsafe extern "C" fn pj_create_ToDoInsert() -> *mut ToDoInsert {
    Box::into_raw(Box::new(ToDoInsert::new()))
}

#[no_mangle]
pub unsafe extern "C" fn pj_create_ToDoQuery() -> *mut ToDoQuery {
    Box::into_raw(Box::new(ToDoQuery::new()))
}
