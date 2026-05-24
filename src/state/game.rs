use crate::{data::{item::Item, perk::Perk}, entity::player::Player, state::game_setup::Stats};

pub mod input;
pub mod ui;

#[derive(Debug)]
pub struct MainGameState {
    player: Player,
}

impl Default for MainGameState {
    fn default() -> Self {
        Self {
            player: Player::default(),
        }
    }
}

impl MainGameState {
    pub fn game_start(
        &mut self,
        name: String,
        paranoia: u8,
        stats: Stats,
        start_item: Item,
        perk: Perk,
    ) {
        self.player = Player::start(name, paranoia, stats, start_item, perk);
    }
}
