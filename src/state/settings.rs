use crate::util::MenuCursor;

pub mod ui;
pub mod input;

#[derive(Debug)]
pub struct SettingsState {
    pub cursor: MenuCursor,
    pub page_idx: usize
}

impl Default for SettingsState {
    fn default() -> Self {
        Self {
            cursor: MenuCursor::new(4),
            page_idx: 1
        }
    }
}

impl SettingsState{
    pub fn increment_page_idx(&mut self) {
       if self.page_idx == 1 {
            self.page_idx = 1
        } else {
            self.page_idx += 1
        }
    }

    pub fn decrement_page_idx(&mut self) {
        if self.page_idx == 1 {
            self.page_idx = 1
        } else {
            self.page_idx -= 1
        }
    }
}
