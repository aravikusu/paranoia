pub mod ui;
pub mod input;

#[derive(Debug)]
pub struct SettingsState {
    pub menu_idx: usize,
    pub page_idx: usize
}

const MENU1_ITEMS: usize = 3;

impl Default for SettingsState {
    fn default() -> Self {
        Self {
            menu_idx: 0,
            page_idx: 1
        }
    }
}

impl SettingsState{
    pub fn increment_menu_idx(&mut self) {
        if self.menu_idx == MENU1_ITEMS {
            self.menu_idx = 0
        } else {
            self.menu_idx += 1
        }
    }

    pub fn decrement_menu_idx(&mut self) {
        if self.menu_idx == 0 {
            self.menu_idx = MENU1_ITEMS
        } else {
            self.menu_idx -= 1
        }
    }

    pub fn increment_page_idx(&mut self) {
        if self.menu_idx == 1 {
            self.menu_idx = 1
        } else {
            self.menu_idx += 1
        }
    }

    pub fn decrement_page_idx(&mut self) {
        if self.menu_idx == 1 {
            self.menu_idx = 1
        } else {
            self.menu_idx -= 1
        }
    }
}