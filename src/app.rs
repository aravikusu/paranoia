use std::error;

use crate::app_settings::AppSettings;
use crate::state::game_setup::GameSetupState;
use crate::state::settings::SettingsState;
use crate::state::title::TitleState;

/// Application result type.
pub type AppResult<T> = Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
pub enum GameState {
    Title = 0,
    GameSetup = 1,
    Game = 2,
    Manual = 3,
    Settings = 4,
}

#[derive(Debug)]
pub struct App<'a> {
    pub game_state: GameState,
    pub running: bool,
    pub settings: AppSettings,

    pub title_state: TitleState,
    pub settings_state: SettingsState,
    pub game_setup_state: GameSetupState<'a>,
}

impl Default for App<'_> {
    fn default() -> Self {
        Self {
            game_state: GameState::Title,
            running: true,
            settings: AppSettings::initialize(),
            title_state: TitleState::default(),
            settings_state: SettingsState::default(),
            game_setup_state: GameSetupState::default(),
        }
    }
}

impl App<'_> {
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
