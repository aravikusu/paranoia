use crossterm::event::{KeyCode, KeyEvent};

use crate::{app::App, screen::Screen};

pub fn handle_input(key_event: KeyEvent, app: &mut App) {
    if app.game_setup_state.selecting_item || app.game_setup_state.selecting_perk {
        match key_event.code {
            KeyCode::Enter => {
                if app.game_setup_state.selecting_item {
                    app.game_setup_state.set_item();
                    app.game_setup_state.toggle_item();
                } else {
                    app.game_setup_state.set_perk();
                    app.game_setup_state.toggle_perk();
                }
            }
            KeyCode::Esc => {
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

    if app.game_setup_state.editing_stats {
        match key_event.code {
            KeyCode::Enter | KeyCode::Esc => {
                app.game_setup_state.toggle_stats();
            },
            KeyCode::Char('a') | KeyCode::Char('A') | KeyCode::Left => {
                app.game_setup_state.stats_cursor.prev();
            }
            KeyCode::Char('d') | KeyCode::Char('D') | KeyCode::Right => {
                app.game_setup_state.stats_cursor.next();
            }
            KeyCode::Char('w') | KeyCode::Char('W') | KeyCode::Up => {
                app.game_setup_state.increment_stat();
            }
            KeyCode::Char('s') | KeyCode::Char('S') | KeyCode::Down => {
                app.game_setup_state.decrement_stat();
            },
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
            // Stat navigation
            (KeyCode::Enter, 1) => {
                app.game_setup_state.toggle_stats();
            }
            
            // Item 
            (KeyCode::Enter, 3) => {
                app.game_setup_state.toggle_item();
                app.game_setup_state.submenu_cursor.update_len(2);
            }
            // Perk
            (KeyCode::Enter, 4) => {
                app.game_setup_state.toggle_perk();
                app.game_setup_state.submenu_cursor.update_len(2);
            }
            // Navigating the menu itself
            (KeyCode::Char('w') | KeyCode::Char('W') | KeyCode::Up, _) => {
                app.game_setup_state.cursor.prev();
            }
            (KeyCode::Char('s') | KeyCode::Char('S') | KeyCode::Down, _) => {
                app.game_setup_state.cursor.next();
            }
            // Exiting out
            (KeyCode::Esc | KeyCode::Backspace, _) => {
                app.change_screen(Screen::Title);
            }
            // Paranoia
            (KeyCode::Char('a') | KeyCode::Char('A') | KeyCode::Left, 2) => {
                app.game_setup_state.decrement_paranoia_level();
            }
            (KeyCode::Char('d') | KeyCode::Char('D') | KeyCode::Right, 2) => {
                app.game_setup_state.increment_paranoia_level();
            }
            (KeyCode::Enter, 5) => {
                app.change_screen(Screen::Game);
                app.main_game_state.game_start(app.game_setup_state.stats.clone());
            }
            _ => {}
        }
    }
}
