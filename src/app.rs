use std::error;

use crate::app_settings::AppSettings;
use crate::data::database::Database;
use crate::screen::Screen;
use crate::state::game_setup::GameSetupState;
use crate::state::settings::SettingsState;
use crate::state::title::TitleState;

/// Application result type.
pub type AppResult<T> = Result<T, Box<dyn error::Error>>;


#[derive(Debug)]
pub struct App {
    pub screen: Screen,
    pub running: bool,
    pub settings: AppSettings,
    pub database: Database,

    pub title_state: TitleState,
    pub settings_state: SettingsState,
    pub game_setup_state: GameSetupState,
}

impl Default for App {
    fn default() -> Self {
        Self {
            screen: Screen::Title,
            running: true,
            settings: AppSettings::default(),
            database: Database::default(),
            title_state: TitleState::default(),
            settings_state: SettingsState::default(),
            game_setup_state: GameSetupState::default(),
        }
    }
}

impl App {
    pub fn new() -> Self {
        let database = Database::load();
        let starting_items = database.get_starting_items();

        Self {
            settings: AppSettings::initialize(),
            database,
            game_setup_state: GameSetupState {
                starting_items,
                ..Default::default()
            },
            ..Default::default()
        }
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Closes the app (big surprise).
    pub fn quit(&mut self) {
        self.settings.save_settings();
        self.running = false;
    }

    pub fn change_screen(&mut self, screen: Screen) {
        self.screen = screen;
    }
}
