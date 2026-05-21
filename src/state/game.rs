use crate::state::game_setup::Stats;

pub mod input;
pub mod ui;

#[derive(Debug)]
pub struct MainGameState {

}

impl Default for MainGameState {
    fn default() -> Self {
        Self {

        }
    }
}

impl MainGameState {
    pub fn game_start(&mut self, _stats: Stats) {

    }
}
