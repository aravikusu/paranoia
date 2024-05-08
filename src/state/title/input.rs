use crossterm::event::{KeyCode, KeyEvent};
use crate::app::{App, GameState};

pub fn handle_input(key_event: KeyEvent, app: &mut App) {
    match key_event.code {
        // Exit application on `Ctrl-C`
        KeyCode::Char('w') | KeyCode::Char('W') | KeyCode::Up => {
            app.title_state.decrement_menu_idx();
        }
        KeyCode::Char('s') | KeyCode::Char('S') | KeyCode::Down => {
            app.title_state.increment_menu_idx();
        }
        KeyCode::Enter => {
            match app.title_state.menu_idx {
                0 => app.change_game_state(GameState::CharacterCreation),
                1 => app.change_game_state(GameState::Settings),
                2 => app.change_game_state(GameState::Manual),
                _ => app.quit()
            }
        }
        _ => {}
    }
}