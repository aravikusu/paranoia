use crate::util::MenuCursor;

pub mod ui;
pub mod input;

#[derive(Debug)]
pub struct TitleState {
    pub cursor: MenuCursor,
}

impl Default for TitleState {
    fn default() -> Self {
        Self {
            cursor: MenuCursor::new(4)
        }
    }
}
