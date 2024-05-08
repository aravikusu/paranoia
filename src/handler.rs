use crate::app::{App, AppResult, GameState};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::state::{title, manual, settings};

pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    // Application-wide events
    match key_event.code {
        // Exit application on `Ctrl-C`
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit();
            }
        }
        _ => {}
    }
    
    match app.game_state {
        GameState::Title => title::input::handle_input(key_event, app),
        GameState::CharacterCreation => {}
        GameState::Game => {}
        GameState::Manual => manual::input::handle_input(key_event, app),
        GameState::Settings => settings::input::handle_input(key_event, app)
    }
    Ok(())
}
