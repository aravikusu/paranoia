use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, BorderType, Paragraph};
use ratatui::widgets::TitlePosition;
use ratatui::widgets::{List, ListItem, ListDirection, HighlightSpacing};

use crate::app_settings::AppTheme;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MenuCursor {
    pub idx: usize,
    len: usize,
}

impl MenuCursor {
    pub fn new(len: usize) -> Self {
        Self {
            idx: 0,
            len
        }
    }

    pub fn update_len(&mut self, len: usize) {
        self.len = len;
    }

    pub fn next(&mut self) {
        self.idx = (self.idx + 1) % self.len;
    }

    pub fn prev(&mut self) {
        self.idx = (self.idx + self.len - 1) % self.len;
    }

    pub fn selected(self) -> usize {
        self.idx
    }

    pub fn len(self) -> usize {
        self.len
    }
}

/// The general outlined Block used everywhere.
pub fn block_preset<'a>(title: impl Into<Line<'a>>) -> Block<'a> {
    Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title(title)
        .title_alignment(Alignment::Left)
        .title_position(TitlePosition::Top)
}

pub fn menu_header<'a>(lines: impl Into<Text<'a>>, color: Color) -> Paragraph<'a> {
    Paragraph::new(lines)
        .style(Style::default().fg(color))
        .alignment(Alignment::Center)
}

pub fn list_preset<'a>(
    items: impl IntoIterator<Item = ListItem<'a>>,
    block: Block <'a>,
    theme: AppTheme
) -> List<'a> {
    List::new(items)
        .block(block)
        .style(Style::default().fg(theme.fg_color()))
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::ITALIC)
                .fg(Color::Black)
                .bg(theme.fg_color())
        )
        .highlight_symbol(">>")
        .repeat_highlight_symbol(true)
        .highlight_spacing(HighlightSpacing::Always)
        .direction(ListDirection::TopToBottom)
}
