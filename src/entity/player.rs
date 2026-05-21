use crate::{data::{item::Item, perk::Perk}, entity::character::Character, state::game_setup::Stats};

#[derive(Debug)]
pub struct Player {
    pub character: Character,
    pub inventory: Vec<Item>,
    pub perk: Perk

}

impl Default for Player {
    fn default() -> Self {
        Self {
            character: Character::default(),
            inventory: vec!(),
            perk: Perk::default(),
        }
    }
}

impl Player {
    pub fn start(&mut self,
        name: String,
        stats: Stats,
        start_item: Item,
        perk: Perk) -> Self {
        Self {
            character: Character::new(name, stats),
            inventory: vec!(start_item),
            perk
        }
    }
}
