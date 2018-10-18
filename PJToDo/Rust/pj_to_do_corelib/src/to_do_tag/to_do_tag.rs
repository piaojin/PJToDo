extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use db::tables::schema::{todotag};

#[repr(C)]
#[derive(
    Serialize, Deserialize, Debug, Default, PartialEq, Queryable, AsChangeset, Identifiable,
)]
#[table_name = "todotag"]
pub struct ToDoTag {
    id: i32,
    tag_name: String,
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Insertable)]
#[table_name = "todotag"]
pub struct ToDoTypeInsert {
    pub tag_name: String,
}
