extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use self::serde::{Deserialize, Serialize};

pub trait PJSerdeDeserialize<'a>: Deserialize<'a> + Serialize {
    type Item: Default + std::fmt::Debug;
    fn new() -> Self::Item {
        Self::Item::default()
    }
}
