use std::cmp;

use crate::data::{item::Item, perk::Perk};
use crate::util::MenuCursor;

pub mod input;
pub mod ui;

// TODO: Move this and extend later
#[derive(Debug, Default, Clone, Copy)]
pub struct Stats {
    pub str: u8,
    pub int: u8,
    pub agi: u8,
    pub luk: u8,
}

impl Stats {
    pub fn new() -> Self {
        Self {
            str: 1,
            int: 1,
            agi: 1,
            luk: 1,
        }
    }
}

#[derive(Debug)]
pub struct GameSetupState {
    pub cursor: MenuCursor,
    pub editing_name: bool,
    pub selecting_item: bool,
    pub selecting_perk: bool,
    pub submenu_cursor: MenuCursor,
    pub starting_items: Vec<Item>,
    pub perks: Vec<Perk>,
    pub stats_cursor: MenuCursor,
    pub stat_points_to_allocate: usize,
    pub editing_stats: bool,

    pub name: String,
    pub paranoia: u8,
    pub item: Option<Item>,
    pub perk: Option<Perk>,
    pub stats: Stats,
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
            stats_cursor: MenuCursor::new(4),
            stat_points_to_allocate: 12,
            paranoia: 0,
            starting_items: vec!(),
            perks: vec!(),
            item: None,
            perk: None,
            stats: Stats::new(),
            editing_stats: false,
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
    
     pub fn set_perk(&mut self) {
        let idx = self.submenu_cursor.selected();
         self.perk = self.perks.get(idx).cloned();
    }

    pub fn increment_stat(&mut self) {
        if self.stat_points_to_allocate == 0 {
            return
        }
        self.stat_points_to_allocate -= 1;
        match self.stats_cursor.selected() {
            0 => self.stats.str = self.stats.str.saturating_add(1),
            1 => self.stats.int = self.stats.int.saturating_add(1),
            2 => self.stats.agi = self.stats.agi.saturating_add(1),
            3 => self.stats.luk = self.stats.luk.saturating_add(1),
            _ => {}
        }
    }
    
    pub fn decrement_stat(&mut self) {
        let mut lowered = false;
        match self.stats_cursor.selected() {
            0 => {
                if self.stats.str == 1 {
                    return;
                }
                lowered = true;
                self.stats.str = self.stats.str.saturating_sub(1)
            }
            1 => {
                if self.stats.int == 1 {
                    return;
                }
                lowered = true;
                self.stats.int = self.stats.int.saturating_sub(1)
            }
            2 => {
                if self.stats.agi == 1 {
                    return;
                }
                lowered = true;
                self.stats.agi = self.stats.agi.saturating_sub(1)
            }
            3 => {
                if self.stats.luk == 1 {
                    return;
                }
                lowered = true;
                self.stats.luk = self.stats.luk.saturating_sub(1)
            }
            _ => {}
        }
        if lowered {
            self.stat_points_to_allocate += 1;
        }
    }

    pub fn toggle_stats(&mut self) {
        self.editing_stats = !self.editing_stats;
    }

    pub fn start_available(&self) -> bool {
        if self.name.is_empty() || self.item.is_none() || self.stat_points_to_allocate > 0 {
            return false
        }
        true
    }
    pub fn clean_up(&self) {
       // todo: clear variables
    }
}
