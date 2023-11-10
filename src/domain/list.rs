use crate::domain::item::Item;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct List {
    pub name: String,
    pub items: Vec<Item>,
}

impl List {
    pub fn new(name: String) -> Self {
        List {
            name,
            items: Vec::new(),
        }
    }
}
