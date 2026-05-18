use crate::app::{App, AppResult, GameState};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::state::{title, manual, settings, game_setup};

pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    // Application-wide events
    if let (KeyCode::Char('c') | KeyCode::Char('C'), KeyModifiers::CONTROL) = (key_event.code, key_event.modifiers) {
        app.quit()
    }
    
    match app.game_state {
        GameState::Title => title::input::handle_input(key_event, app),
        GameState::GameSetup => game_setup::input::handle_input(key_event, app),
        GameState::Game => {}
        GameState::Manual => manual::input::handle_input(key_event, app),
        GameState::Settings => settings::input::handle_input(key_event, app)
    }
    Ok(())
}
