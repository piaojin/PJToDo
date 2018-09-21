extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use db::tables::schema::{todotag};

#[derive(Serialize, Deserialize, Debug, Default)]
#[derive(Insertable)]
#[table_name = "todotag"]
pub struct ToDoTag {
    id: i32,
    tag_name: String
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
#[derive(Insertable)]
#[table_name = "todotag"]
pub struct ToDoTagInsert {
    pub tag_name: String,
}