use std::cmp;

use crate::{app::App, data::item::Item, util::MenuCursor};

pub mod input;
pub mod ui;

#[derive(Debug)]
pub struct GameSetupState {
    pub cursor: MenuCursor,
    pub editing_name: bool,
    pub selecting_item: bool,
    pub selecting_perk: bool,
    pub submenu_cursor: MenuCursor,
    pub name: String,
    pub paranoia: i32,
    pub starting_items: Vec<Item>
}

impl Default for GameSetupState {
    fn default() -> Self {
        Self {
            cursor: MenuCursor::new(4),
            editing_name: false,
            selecting_item: false,
            selecting_perk: false,
            name: String::new(),
            submenu_cursor: MenuCursor::new(0),
            paranoia: 0,
            starting_items: vec!(),
        }
    }
}

impl GameSetupState {
    pub fn load(&mut self, app: &App) {
        self.starting_items = app.database.get_starting_items();
    }

    pub fn toggle_edit(&mut self) {
        self.editing_name = !self.editing_name;
    }
    pub fn toggle_item(&mut self) {
        self.selecting_item = !self.selecting_item;
    }

    pub fn toggle_perk(&mut self) {
        self.selecting_perk = !self.selecting_perk;
    }

    pub fn increment_paranoia_level(&mut self) {
        self.paranoia = cmp::min(100, self.paranoia + 1);
    }
    pub fn decrement_paranoia_level(&mut self) {
        self.paranoia = cmp::max(0, self.paranoia - 1);
    }
}
