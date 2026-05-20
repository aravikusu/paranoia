use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone )]
pub struct Item {
    pub key: String,
    pub name: String,
    pub description: String,
}
