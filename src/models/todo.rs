use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Todo {
    pub title: String,
    pub desc: String,
}