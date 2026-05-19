use crossterm::event::{KeyCode, KeyEvent};

use crate::{app::App, screen::Screen};

pub fn handle_input(key_event: KeyEvent, app: &mut App) {
    if app.game_setup_state.selecting_item || app.game_setup_state.selecting_perk {
        match key_event.code {
            KeyCode::Enter | KeyCode::Esc => {
                if app.game_setup_state.selecting_item {
                    app.game_setup_state.toggle_item();
                } else {
                    app.game_setup_state.toggle_perk();
                }
            }
            KeyCode::Char('w') | KeyCode::Char('W') | KeyCode::Up => {
                app.game_setup_state.submenu_cursor.prev();
            }
            KeyCode::Char('s') | KeyCode::Char('S') | KeyCode::Down => {
                app.game_setup_state.submenu_cursor.next();
            }
            _ => {}
        }
        return;
    }

    if app.game_setup_state.editing_name {
        match key_event.code {
            KeyCode::Enter | KeyCode::Esc => {
                app.game_setup_state.toggle_edit();
            },
            KeyCode::Char(c) => {
                app.game_setup_state.name.push(c);
            },
            KeyCode::Backspace => {
                app.game_setup_state.name.pop();
            },
            _ => {}
        }
    } else {
        match (key_event.code, app.game_setup_state.cursor.selected()) {
            (KeyCode::Enter, 0) => {
                app.game_setup_state.toggle_edit();
            }
            (KeyCode::Enter, 2) => {
                app.game_setup_state.toggle_item();
                app.game_setup_state.submenu_cursor.update_len(2);
            }
            (KeyCode::Enter, 3) => {
                app.game_setup_state.toggle_perk();
                app.game_setup_state.submenu_cursor.update_len(2);
            }
            (KeyCode::Char('w') | KeyCode::Char('W') | KeyCode::Up, _) => {
                app.game_setup_state.cursor.prev();
            }
            (KeyCode::Char('s') | KeyCode::Char('S') | KeyCode::Down, _) => {
                app.game_setup_state.cursor.next();
            }
            (KeyCode::Esc | KeyCode::Backspace, _) => {
                app.change_screen(Screen::Title);
            }
            (KeyCode::Char('a') | KeyCode::Char('A') | KeyCode::Left, 1) => {
                app.game_setup_state.decrement_paranoia_level();
            }
            (KeyCode::Char('d') | KeyCode::Char('D') | KeyCode::Right, 1) => {
                app.game_setup_state.increment_paranoia_level();
            }
            _ => {}
        }
    }
}
