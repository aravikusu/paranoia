use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, BorderType, Paragraph};
use ratatui::widgets::block::Position;

/// The general outlined Block used everywhere.
pub fn block_preset<'a>(title: String) -> Block<'a> {
    Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title(title)
        .title_alignment(Alignment::Left)
        .title_position(Position::Top)
}

pub fn menu_header(lines: Vec<Line>, color: Color) -> Paragraph {
    Paragraph::new(lines)
        .style(Style::default().fg(color))
        .alignment(Alignment::Center)
}