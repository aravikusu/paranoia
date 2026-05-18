use crossterm::event::KeyEvent;
use ratatui::{Frame, layout::Rect, text::Line};

use crate::{app::App, state::{game_setup, manual, settings, title}};

#[derive(Debug, Copy, Clone)]
pub enum Screen {
    Title,
    GameSetup,
    Game,
    Manual,
    Settings,
}

impl Screen {
    pub fn handle_input(self, key: KeyEvent, app: &mut App ) {
        match self {
            Screen::Title => title::input::handle_input(key, app),
            Screen::GameSetup => game_setup::input::handle_input(key, app),
            Screen::Game => todo!(),
            Screen::Manual => manual::input::handle_input(key, app),
            Screen::Settings => settings::input::handle_input(key, app),
        }
    }

    pub fn render(self, app: &mut App, frame: &mut Frame, main_layout: [Rect; 1]) {
        match self {
            Screen::Title => title::ui::layout(app, frame, main_layout),
            Screen::GameSetup => game_setup::ui::layout(app, frame, main_layout),
            Screen::Game => todo!(),
            Screen::Manual => manual::ui::layout(app, frame, main_layout),
            Screen::Settings => settings::ui::layout(app, frame, main_layout),
        }
    }

    pub fn instructions(self, app: &mut App) -> Line<'static> {
        match self {
            Screen::Title => title::ui::instructions(app),
            Screen::GameSetup => game_setup::ui::instructions(app),
            Screen::Game => todo!(),
            Screen::Manual => manual::ui::instructions(app),
            Screen::Settings => settings::ui::instructions(app),
        }
    }
}
