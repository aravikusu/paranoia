pub mod ui;
pub mod input;

#[derive(Debug)]
pub struct SettingsState {
    pub menu_idx: usize,
    pub page_idx: usize
}

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
        if self.menu_idx == 1 {
            self.menu_idx = 0
        } else {
            self.menu_idx += 1
        }
    }

    pub fn decrement_menu_idx(&mut self) {
        if self.menu_idx == 0 {
            self.menu_idx = 1
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