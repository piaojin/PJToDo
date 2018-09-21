table! {
    todotype (id) {
        id -> Integer,
        to_do_id -> Integer,
        type_name -> Text,
    }
}

table! {
    todotag (id) {
        id -> Integer,
        to_do_id -> Integer,
        tag_name -> Text,
    }
}

table! {
    use diesel::sql_types::*;
    use to_do::to_do::ToDoStateType;
    todo (id) {
        id -> Integer,
        content -> Text,
        title -> Text,
        due_time -> Text,
        remind_time -> Text,
        create_time -> Text,
        update_time -> Text,
        to_do_type_id -> Integer,
        to_do_tag_id -> Integer,
        state -> ToDoStateType,
    }
}

// pub struct ToDo<'a> {
//     pub id: i64,
//     pub content: &'a str, //待办事项内容
//     pub title: &'a str, //待办事项标题
//     pub due_time: &'a str, //到期时间
//     pub remind_time: &'a str, //提醒时间
//     pub create_time: &'a str, //创建时间
//     pub update_time: &'a str, //更新时间
//     pub to_do_tag: ToDoTag, //标签
//     pub to_do_type: ToDoType, //分类
//     pub state: ToDoState //状态
// }