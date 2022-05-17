use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ItemPair {
    pub name: String,
    pub cook_time: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateItem {
    pub tid: i32,
    pub items: Vec<ItemPair>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteItem {
    pub tid: i32,
    pub item: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateItem {
    pub name: String,
    pub cook_time: i32,
}