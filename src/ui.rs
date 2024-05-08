use ratatui::{
    layout::Alignment,
    widgets::{Block, BorderType},
    Frame,
};
use ratatui::prelude::Style;
use ratatui::style::Color;
use ratatui::widgets::block::Position;

use crate::app::{App, GameState};
use crate::state::{title, manual, settings};


/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui-org/ratatui/tree/master/examples

    // The bottom of the main frame always displays the controls for the current screen.
    // An "intuitive" way of making sure the player knows what buttons do what.
    let instructions = match app.game_state {
        GameState::Title => title::ui::instructions(),
        GameState::Manual => manual::ui::instructions(),
        GameState::Settings => settings::ui::instructions(),
        _ => title::ui::instructions(),
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
        .style(Style::default().fg(Color::LightYellow));
    
    // Whatever goes inside the main frame is decided by the game state.
    match app.game_state {
        GameState::Title => title::ui::layout(app, frame, block),
        GameState::Manual => manual::ui::layout(app, frame, block),
        GameState::Settings => settings::ui::layout(app, frame, block),
        _ => title::ui::layout(app, frame, block)
    };
}