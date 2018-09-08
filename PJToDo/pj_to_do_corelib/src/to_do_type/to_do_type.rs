extern crate serde_derive;
extern crate serde;
extern crate serde_json;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ToDoType {
    pub id: i64,
    pub type_name: String
}