use tui_textarea::TextArea;

pub mod input;
pub mod ui;

#[derive(Debug, Default)]
pub struct GameSetupState<'a> {
    pub menu_idx: usize,
    pub editing_name: bool,
    pub item_list_idx: usize,

    pub name_input: TextArea<'a>,
    pub paranoia: i32,
    pub starting_item: String,
}

impl GameSetupState<'_> {
    pub fn toggle_edit(&mut self) {
        self.editing_name = !self.editing_name;
    }

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