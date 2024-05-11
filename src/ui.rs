use ratatui::{
    Frame,
    layout::Alignment,
    widgets::{Block, BorderType},
};
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::prelude::{Color, Rect, Style};
use ratatui::widgets::block::Position;

use crate::app::{App, GameState};
use crate::app_settings::AppTheme;
use crate::state::{manual, settings, title, game_setup};

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
        GameState::GameSetup => game_setup::ui::instructions(app),
        _ => title::ui::instructions(app),
    };

    // The main frame around the entire game.
    let main_frame_block = Block::bordered()
        .title("PARANOIA")
        .title(
            instructions
                .alignment(Alignment::Center)
                .position(Position::Bottom),
        )
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded)
        .style(Style::default().fg(AppTheme::fg_color(&app.settings.theme)).bg(Color::Black));

    let main_layout: [Rect;1] = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Fill(1)
            ]
        )
        .areas(main_frame_block.inner(frame.size()));
    
    frame.render_widget(main_frame_block, frame.size());

    // Whatever goes inside the main frame is decided by the game state.
    match app.game_state {
        GameState::Title => title::ui::layout(app, frame, main_layout),
        GameState::Manual => manual::ui::layout(app, frame, main_layout),
        GameState::Settings => settings::ui::layout(app, frame, main_layout),
        GameState::GameSetup => game_setup::ui::layout(app, frame, main_layout),
        _ => game_setup::ui::layout(app, frame, main_layout)
    };
}