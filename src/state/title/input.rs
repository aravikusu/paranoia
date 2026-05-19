use crossterm::event::{KeyCode, KeyEvent};

use crate::{app::App, screen::Screen};

pub fn handle_input(key_event: KeyEvent, app: &mut App) {
    match key_event.code {
        // Exit application on `Ctrl-C`
        KeyCode::Char('w') | KeyCode::Char('W') | KeyCode::Up => {
            app.title_state.cursor.prev();
        }
        KeyCode::Char('s') | KeyCode::Char('S') | KeyCode::Down => {
            app.title_state.cursor.next();
        }
        KeyCode::Enter => {
            match app.title_state.cursor.selected() {
                0 => app.change_screen(Screen::GameSetup),
                1 => app.change_screen(Screen::Settings),
                2 => app.change_screen(Screen::Manual),
                _ => app.quit()
            }
        }
        _ => {}
    }
}
