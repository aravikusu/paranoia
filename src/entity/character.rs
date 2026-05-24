use crate::state::game_setup::Stats;


#[derive(Debug)]
pub struct Character {
    pub name: String,
    pub max_hp: u8,
    pub cur_hp: u8,
    pub stats: Stats,
}

impl Default for Character {
    fn default() -> Self {
        Self {
            name: String::from("placeholder"),
            stats: Stats::default(),
            cur_hp: 10,
            max_hp: 10,
        }
    }
}

impl Character {
    pub fn new(name: String, stats: Stats) -> Self {
        Self {
            name,
            stats,
            cur_hp: 10,
            max_hp: 10,
        }
    }
}
