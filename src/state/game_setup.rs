use std::cmp;

use crate::util::MenuCursor;

pub mod input;
pub mod ui;

#[derive(Debug)]
pub struct GameSetupState {
    pub cursor: MenuCursor,
    pub editing_name: bool,
    pub item_list_idx: usize,

    pub name: String,
    pub paranoia: i32,
    pub starting_item: String,
}

impl Default for GameSetupState {
    fn default() -> Self {
        Self {
            cursor: MenuCursor::new(4),
            editing_name: false,
            item_list_idx: 0,
            name: String::new(),
            paranoia: 0,
            starting_item: String::new(),
        }
    }
}

impl GameSetupState {
    pub fn toggle_edit(&mut self) {
        self.editing_name = !self.editing_name;
    }

    pub fn increment_paranoia_level(&mut self) {
        self.paranoia = cmp::min(100, self.paranoia + 1);
    }
    pub fn decrement_paranoia_level(&mut self) {
        self.paranoia = cmp::max(0, self.paranoia - 1);
    }
}
