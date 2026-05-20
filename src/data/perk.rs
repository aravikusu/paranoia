use serde::{Deserialize, Serialize};

use crate::data::database::Describable;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Perk {
    pub key: String,
    pub name: String,
    pub description: String,
}

impl Describable for Perk {
    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        &self.description
    }
}
