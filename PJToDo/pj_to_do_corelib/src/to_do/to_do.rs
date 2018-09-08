extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use common::pj_serialize::PJSerdeDeserialize;
use to_do_tag::to_do_tag::ToDoTag;
use to_do_type::to_do_type::ToDoType;

#[derive(Serialize, Deserialize, Debug)]
pub enum ToDoState {
    Determined,
    InProgress,
    Finished,
    Overdue
}

impl Default for ToDoState {
    fn default() -> ToDoState { 
        ToDoState::Determined 
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ToDo<'a> {
    pub id: i64,
    pub content: &'a str, //待办事项内容
    pub title: &'a str, //待办事项标题
    pub due_time: &'a str, //到期时间
    pub remind_time: &'a str, //提醒时间
    pub create_time: &'a str, //创建时间
    pub update_time: &'a str, //更新时间
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