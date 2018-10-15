table! {
    todotype (id) {
        id -> Integer,
        type_name -> Text,
    }
}

table! {
    todotag (id) {
        id -> Integer,
        tag_name -> Text,
    }
}

table! {
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
        state -> Integer,
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

lazy_static! {
    pub static ref Table_ToDoType: String = "todotype".to_string();
    pub static ref Table_ToDoTag: String = "todotag".to_string();
    pub static ref Table_ToDo: String = "todo".to_string();

    pub static ref Table_ToDoType_Create_Sql: String = "CREATE TABLE IF NOT EXISTS `todotype` (`id`	INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,`type_name`	TEXT NOT NULL DEFAULT \"\" UNIQUE);".to_string();

    pub static ref Table_ToDoTag_Create_Sql: String = "CREATE TABLE IF NOT EXISTS `todotag` (`id`	INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,`tag_name`	TEXT NOT NULL DEFAULT \"\" UNIQUE);".to_string();

    pub static ref Table_ToDo_Create_Sql: String = "CREATE TABLE IF NOT EXISTS `todo` (`id`	INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,`title`	TEXT NOT NULL DEFAULT \"\",`content`	TEXT NOT NULL DEFAULT \"\",`due_time`	TEXT NOT NULL,`remind_time`	TEXT NOT NULL,`create_time`	TEXT NOT NULL,`update_time`	TEXT NOT NULL DEFAULT \"\",`to_do_tag_id`	INTEGER NOT NULL DEFAULT -1,`to_do_type_id`	INTEGER NOT NULL DEFAULT -1,`state`	INTEGER NOT NULL DEFAULT 0,CONSTRAINT `fk_to_do_type` FOREIGN KEY(`to_do_type_id`) REFERENCES `todotype`(`id`),CONSTRAINT `fk_to_do_tag` FOREIGN KEY(`to_do_tag_id`) REFERENCES `todotag`(`id`));".to_string();
}