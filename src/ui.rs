use ratatui::{
    Frame,
    layout::Alignment,
    widgets::{Block, BorderType},
};
use ratatui::prelude::{Color, Style};
use ratatui::widgets::block::Position;

use crate::app::{App, GameState};
use crate::app_settings::AppTheme;
use crate::state::{manual, settings, title};

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui-org/ratatui/tree/master/examples

    // The bottom of the main frame always displays the controls for the current screen.
    // An "intuitive" way of making sure the player knows what buttons do what.
    let instructions = match app.game_state {
        GameState::Title => title::ui::instructions(app),
        GameState::Manual => manual::ui::instructions(app),
        GameState::Settings => settings::ui::instructions(app),
        _ => title::ui::instructions(app),
    };

    // The main frame around the entire game.
    let block = Block::bordered()
        .title("PARANOIA")
        .title(
            instructions
                .alignment(Alignment::Center)
                .position(Position::Bottom),
        )
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded)
        .style(Style::default().fg(AppTheme::fg_color(&app.settings.theme)).bg(Color::Black));

    // Whatever goes inside the main frame is decided by the game state.
    match app.game_state {
        GameState::Title => title::ui::layout(app, frame, block),
        GameState::Manual => manual::ui::layout(app, frame, block),
        GameState::Settings => settings::ui::layout(app, frame, block),
        _ => title::ui::layout(app, frame, block)
    };
}