use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::prelude::{Alignment, Line, Styled, Stylize};
use ratatui::style::Style;
use ratatui::widgets::{Block, Borders, BorderType, Paragraph};
use ratatui::widgets::block::{Position, Title};

use crate::app::App;
use crate::app_settings::AppTheme;
use crate::util::menu_header;

const MANUAL_TEXT: &str = "According to all known laws
of aviation,


there is no way a bee
should be able to fly.


Its wings are too small to get
its fat little body off the ground.


The bee, of course, flies anyway


because bees don't care
what humans think is impossible.";

pub fn instructions(app: &App) -> Title<'static> {
    Title::from(Line::from(vec![
        " change page ".into(),
        "<WASD/ARROW KEYS> ".set_style(Style::default().fg(AppTheme::highlight_color(&app.settings.theme))).bold(),
        " go back ".into(),
        "<ESC/BACKSPACE> ".set_style(Style::default().fg(AppTheme::highlight_color(&app.settings.theme))).bold(),
    ]))
}

pub fn layout(app: &mut App, frame: &mut Frame, main_layout: [Rect;1]) {
    let inner = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(5),
                Constraint::Percentage(5),
                Constraint::Percentage(5),
                Constraint::Fill(1),
                Constraint::Percentage(5),
            ]
        )
        .split(main_layout[0]);

    let header = vec![
        "-paranoia manual-".bold().into(),
        "this is the manual for paranoia. read about the what's, the how's, and the why's here.".italic().into(),
    ];
    frame.render_widget(
        menu_header(header, AppTheme::fg_color(&app.settings.theme)),
        inner[1]);

    let main_part = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(20),
                Constraint::Percentage(60),
                Constraint::Percentage(20)
            ]
        )
        .split(inner[3]);

    let page = Title::from(Line::from(vec!["page 1/1".into()]));

    let manual_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title("intro")
        .title(
            page
                .alignment(Alignment::Right)
                .position(Position::Bottom),
        )
        .title_alignment(Alignment::Left)
        .title_position(Position::Top);

    frame.render_widget(Paragraph::new(MANUAL_TEXT).alignment(Alignment::Center).block(manual_block.clone()), main_part[1]);
}