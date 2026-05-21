use crossterm::event::{KeyCode, KeyEvent};

use crate::{app::App, screen::Screen};

pub fn handle_input(key_event: KeyEvent, app: &mut App) {
    match key_event.code {
        KeyCode::Esc => app.change_screen(Screen::Title) ,
        _ => {}
    }
}
