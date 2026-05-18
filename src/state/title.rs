pub mod ui;
pub mod input;

#[derive(Debug, Default)]
pub struct TitleState {
    pub menu_idx: usize
}

impl TitleState {
    pub fn increment_menu_idx(&mut self) {
        if self.menu_idx == 3 {
            self.menu_idx = 0
        } else {
            self.menu_idx += 1
        }
    }

    pub fn decrement_menu_idx(&mut self) {
        if self.menu_idx == 0 {
            self.menu_idx = 3
        } else {
            self.menu_idx -= 1
        }
    }
}
