use ratatui::{
    Frame,
    layout::Alignment,
    widgets::{Block, BorderType},
};
use ratatui::prelude::{Color, Style};
use crate::app::App;

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui-org/ratatui/tree/master/examples

    // The bottom of the main frame always displays the controls for the current screen.
    // An "intuitive" way of making sure the player knows what buttons do what.
    let instructions = app.screen.instructions(app);

    // The main frame around the entire game.
    let main_frame_block = Block::bordered()
        .title("PARANOIA")
        .title_bottom(instructions.centered())
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded)
        .style(Style::default().fg(app.settings.theme.fg_color()).bg(Color::Black));

    let inner = main_frame_block.inner(frame.area());
    frame.render_widget(main_frame_block, frame.area());

    // Whatever goes inside the main frame is decided by the game state.
    app.screen.render(app, frame, inner);
}
