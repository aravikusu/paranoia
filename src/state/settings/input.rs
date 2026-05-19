use crossterm::event::{KeyCode, KeyEvent};

use crate::app::App;
use crate::app_settings::AppTheme;
use crate::screen::Screen;

pub fn handle_input(key_event: KeyEvent, app: &mut App) {
    match (key_event.code, app.settings_state.page_idx) {
        // Exit application on `Ctrl-C`
        (KeyCode::Char('w') | KeyCode::Char('W') | KeyCode::Up, _) => {
            app.settings_state.cursor.prev();
        }
        (KeyCode::Char('s') | KeyCode::Char('S') | KeyCode::Down, _) => {
            app.settings_state.cursor.next();
        }
        (KeyCode::Esc | KeyCode::Backspace, _) => {
            app.change_screen(Screen::Title);
        }

        (KeyCode::Enter, 1) => {
            match app.settings_state.cursor.selected() {
                0 => app.settings.change_theme(AppTheme::Default),
                1 => app.settings.change_theme(AppTheme::Vaporwave),
                2 => app.settings.change_theme(AppTheme::Piccolo),
                _ => app.settings.change_theme(AppTheme::Monochrome),
            }
        }
        _ => {}
    }
}
