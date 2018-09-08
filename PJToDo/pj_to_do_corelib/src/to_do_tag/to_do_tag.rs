extern crate serde_derive;
extern crate serde;
extern crate serde_json;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ToDoTag {
    id: i64,
    tag_name: String
}