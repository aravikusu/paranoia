use crate::state::game_setup::Stats;


#[derive(Debug)]
pub struct Character {
    pub name: String,
    pub stats: Stats,
}

impl Default for Character {
    fn default() -> Self {
        Self {
            name: String::from("placeholder"),
            stats: Stats::default(),
        }
    }
}

impl Character {
    pub fn new(name: String, stats: Stats) -> Self {
        Self {
            name,
            stats,
        }
    }
}
