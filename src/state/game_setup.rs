use std::cmp;

use crate::data::{item::Item, perk::Perk};
use crate::util::MenuCursor;

pub mod input;
pub mod ui;

#[derive(Debug)]
pub struct GameSetupState {
    pub cursor: MenuCursor,
    pub editing_name: bool,
    pub selecting_item: bool,
    pub selecting_perk: bool,
    pub submenu_cursor: MenuCursor,
    pub starting_items: Vec<Item>,
    pub perks: Vec<Perk>,

    pub name: String,
    pub paranoia: i32,
    pub item: Option<Item>,
    pub perk: Option<Perk>,
}

impl Default for GameSetupState {
    fn default() -> Self {
        Self {
            cursor: MenuCursor::new(6),
            editing_name: false,
            selecting_item: false,
            selecting_perk: false,
            name: String::new(),
            submenu_cursor: MenuCursor::new(0),
            paranoia: 0,
            starting_items: vec!(),
            perks: vec!(),
            item: None,
            perk: None,
        }
    }
}

impl GameSetupState {
    pub fn load(&mut self, starting_items: Vec<Item>, perks: Vec<Perk>) {
        self.starting_items = starting_items;
        self.perks = perks;
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

    pub fn set_item(&mut self) {
        let idx = self.submenu_cursor.selected();
         self.item = self.starting_items.get(idx).cloned();
    }

    pub fn start_available(&self) -> bool {
        if self.name.is_empty() || self.item.is_none() {
            return false
        }
        true
    }
}
