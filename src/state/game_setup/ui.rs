use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::prelude::{Alignment, Line, Styled, Stylize};
use ratatui::style::Style;
use ratatui::widgets::block::{Position, Title};
use ratatui::widgets::Paragraph;

use crate::app::App;
use crate::app_settings::AppTheme;
use crate::util::{block_preset, menu_header};

pub fn instructions(app: &App) -> Title<'static> {
    Title::from(Line::from(vec![
        " navigate ".into(),
        "<WASD/ARROW KEYS> ".set_style(Style::default().fg(AppTheme::highlight_color(&app.settings.theme))).bold(),
        " go back ".into(),
        "<ESC/BACKSPACE> ".set_style(Style::default().fg(AppTheme::highlight_color(&app.settings.theme))).bold(),
    ]))
}

pub fn layout(app: &mut App, frame: &mut Frame, main_block: [Rect; 1]) {
    let inner = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(1),
                Constraint::Length(5),
                Constraint::Fill(1),
                Constraint::Length(15),
                Constraint::Fill(1),
                Constraint::Percentage(20),
            ]
        )
        .split(main_block[0]);

    let header = vec![
        "-game setup-".bold().into(),
        "setup your game.".italic().into(),
    ];
    frame.render_widget(
        menu_header(header, AppTheme::fg_color(&app.settings.theme)),
        inner[1]);

    let main_part = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Fill(1),
                Constraint::Length(30),
                Constraint::Fill(1)
            ]
        )
        .split(inner[3]);

    let settings_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Fill(1),
            ]
        )
        .split(main_part[1]);

    let name_block = block_preset("name".into());
    frame.render_widget(Paragraph::new("text").alignment(Alignment::Left).block(name_block), settings_layout[0]);

    let paranoia_block = block_preset("paranoia level".into())
        .title(
            Title::from("0")
                .alignment(Alignment::Right)
                .position(Position::Bottom),
        );
    frame.render_widget(Paragraph::new("").alignment(Alignment::Left).block(paranoia_block), settings_layout[1]);

    let item_block = block_preset("start item".into());
    frame.render_widget(Paragraph::new("").alignment(Alignment::Left).block(item_block), settings_layout[2]);

    let perk_block = block_preset("start perk".into());
    frame.render_widget(Paragraph::new("").alignment(Alignment::Left).block(perk_block), settings_layout[3]);

    let info_block = block_preset("info box".into());
    frame.render_widget(Paragraph::new("info regarding each field will be shown here.").alignment(Alignment::Center).block(info_block), inner[5]);
}