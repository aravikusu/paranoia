pub mod input;
pub mod ui;

#[derive(Debug, Default)]
pub struct GameSetupState {
    pub menu_idx: usize,
    pub item_list_idx: usize,
    
    pub name: String,
    pub paranoia: i32,
    pub starting_item: String,
}