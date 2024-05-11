use crossterm::event::{KeyCode, KeyEvent};

use crate::app::{App, GameState};

pub fn handle_input(key_event: KeyEvent, app: &mut App) {
    if app.game_setup_state.editing_name {
        match key_event.code {
            KeyCode::Enter | KeyCode::Esc => {
                app.game_setup_state.toggle_edit();
            },
            _ => {
                app.game_setup_state.name_input.input(key_event);
            },
        }
    } else {
        match key_event.code {
            KeyCode::Enter => {
                if app.game_setup_state.menu_idx == 0 {
                    app.game_setup_state.toggle_edit();
                }
            }
            KeyCode::Char('w') | KeyCode::Char('W') | KeyCode::Up => {
                app.game_setup_state.decrement_menu_idx();
            }
            KeyCode::Char('s') | KeyCode::Char('S') | KeyCode::Down => {
                app.game_setup_state.increment_menu_idx();
            }
            KeyCode::Esc | KeyCode::Backspace => {
                app.change_game_state(GameState::Title);
            }
            _ => {}
        }
    }
}