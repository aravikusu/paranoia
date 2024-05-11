use crossterm::event::{KeyCode, KeyEvent};

use crate::app::{App, GameState};

pub fn handle_input(key_event: KeyEvent, app: &mut App) {
    match key_event.code {
        // Exit application on `Ctrl-C`
        KeyCode::Char('w') | KeyCode::Char('W') | KeyCode::Up => {
            //app.settings_state.decrement_menu_idx();
        }
        KeyCode::Char('s') | KeyCode::Char('S') | KeyCode::Down => {
            //app.settings_state.increment_menu_idx();
        }
        KeyCode::Esc | KeyCode::Backspace => {
            app.change_game_state(GameState::Title);
        }
        _ => {}
    }
}