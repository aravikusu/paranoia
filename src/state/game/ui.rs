use ratatui::{Frame, layout::{Constraint, Direction, Layout, Rect}, style::{Style, Styled, Stylize}, text::Line, widgets::{Block, Borders, Paragraph}};

use crate::app::App;

pub fn instructions(app: &App) -> Line<'static> {
    Line::from(vec![
        " select ".into(),
        "<ENTER> ".set_style(Style::default().fg(app.settings.theme.highlight_color())).bold(),
        " navigate ".into(),
        "<WASD/ARROW KEYS> ".set_style(Style::default().fg(app.settings.theme.highlight_color())).bold(),
    ])
}

pub fn layout(app: &App, frame: &mut Frame, main_layout: Rect) {
    let inner = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(10),
                Constraint::Percentage(50),
                Constraint::Percentage(5),
                Constraint::Percentage(35)
            ]
        )
        .split(main_layout);


    frame.render_widget(
        Paragraph::new("goo")
            .block(Block::new().borders(Borders::empty()))
            .style(Style::default().fg(app.settings.theme.fg_color()))
            .centered(),
        inner[1],
    );
}

