use std::error;

use crate::app_settings::AppSettings;
use crate::state::settings::SettingsState;
use crate::state::title::TitleState;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
pub enum GameState {
    Title = 0,
    GameSetup = 1,
    Game = 2,
    Manual = 3,
    Settings = 4,
}

#[derive(Debug)]
pub struct App {
    pub game_state: GameState,
    pub running: bool,
    pub settings: AppSettings,

    pub title_state: TitleState,
    pub settings_state: SettingsState,
}

impl Default for App {
    fn default() -> Self {
        Self {
            game_state: GameState::Title,
            running: true,
            settings: AppSettings::initialize(),
            title_state: TitleState::default(),
            settings_state: SettingsState::default(),
        }
    }
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Closes the app (big surprise).
    pub fn quit(&mut self) {
        self.settings.save_settings();
        self.running = false;
    }

    pub fn change_game_state(&mut self, state: GameState) {
        self.game_state = state;
    }
}
