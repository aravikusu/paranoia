use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, HighlightSpacing, List, ListDirection, ListItem, ListState, Paragraph};
use ratatui::widgets::block::Title;
use crate::app::App;

pub fn instructions() -> Title<'static> {
    Title::from(Line::from(vec![
        " toggle ".into(),
        "<ENTER> ".light_red().bold(),
        " navigate ".into(),
        "<WASD/ARROW KEYS> ".light_red().bold(),
        " go back ".into(),
        "<ESC/BACKSPACE> ".light_red().bold(),
    ]))
}

pub fn layout(_app: &mut App, frame: &mut Frame, main_block: Block) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(100)
            ]
        )
        .split(main_block.inner(frame.size()));
    frame.render_widget(main_block, frame.size());

    frame.render_widget(
        Paragraph::new("wow imagine some cool settings here")
            .style(Style::default().fg(Color::LightYellow))
            .alignment(Alignment::Center),
        chunks[0]);
}