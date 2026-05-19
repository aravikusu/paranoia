use crate::util::MenuCursor;

pub mod ui;
pub mod input;

#[derive(Debug)]
pub struct SettingsState {
    pub cursor: MenuCursor,
    pub page_cursor: MenuCursor,
}

impl Default for SettingsState {
    fn default() -> Self {
        Self {
            cursor: MenuCursor::new(4),
            page_cursor: MenuCursor::new(0),
        }
    }
}
