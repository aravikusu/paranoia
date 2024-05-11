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
    match app.game_setup_state.menu_idx {
        0 | 2 | 3 => {
            Title::from(Line::from(vec![
                " navigate ".into(),
                "<W/A/UP/DOWN> ".set_style(Style::default().fg(AppTheme::highlight_color(&app.settings.theme))).bold(),
                if app.game_setup_state.editing_name { " deselect ".into() } else { " select ".into() },
                "<ENTER> ".set_style(Style::default().fg(AppTheme::highlight_color(&app.settings.theme))).bold(),
                if app.game_setup_state.editing_name { " deselect ".into() } else { " go back ".into() },
                "<ESC> ".set_style(Style::default().fg(AppTheme::highlight_color(&app.settings.theme))).bold(),
            ]))
        }
        _ => Title::from(Line::from(vec![
            " navigate ".into(),
            "<W/A/UP/DOWN> ".set_style(Style::default().fg(AppTheme::highlight_color(&app.settings.theme))).bold(),
            " increment/decrement ".into(),
            "<A/D/LEFT/RIGHT> ".set_style(Style::default().fg(AppTheme::highlight_color(&app.settings.theme))).bold(),
            " go back ".into(),
            "<ESC> ".set_style(Style::default().fg(AppTheme::highlight_color(&app.settings.theme))).bold(),
        ]))
    }
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
    frame.render_widget(
        menu_block_text(
            app.game_setup_state.name_input.lines()[0].as_str().into(),
            app,
            0,
        ).block(name_block), settings_layout[0]);

    let paranoia_block = block_preset("paranoia level".into())
        .title(
            Title::from("0")
                .alignment(Alignment::Right)
                .position(Position::Bottom),
        );

    frame.render_widget(
        menu_block_text(
            "".into(),
            app,
            1,
        ).block(paranoia_block), settings_layout[1]);

    let item_block = block_preset("start item".into());
    frame.render_widget(
        menu_block_text(
            "".into(),
            app,
            2,
        ).block(item_block), settings_layout[2]);

    let perk_block = block_preset("start perk".into());
    frame.render_widget(
        menu_block_text(
            "".into(),
            app,
            3,
        ).block(perk_block), settings_layout[3]);

    let info_block = block_preset("info box".into());
    frame.render_widget(info_block_text(app).alignment(Alignment::Center).block(info_block), inner[5]);
}

fn menu_block_text<'a>(text: String, app: &App, menu_idx: usize) -> Paragraph<'a> {
    let mut paragraph = Paragraph::new(text)
        .alignment(Alignment::Left);

    if app.game_setup_state.menu_idx == menu_idx {
        paragraph = paragraph
            .style(Style::default().bg(
                if app.game_setup_state.editing_name { AppTheme::highlight_color(&app.settings.theme) } else { AppTheme::fg_color(&app.settings.theme) }
            ))
            .black();
    }
    paragraph
}

fn info_block_text<'a>(app: &App) -> Paragraph<'a> {
    let text = match app.game_setup_state.menu_idx {
        0 => "your... name. what everyone will refer to you as\nyou know... a name..",
        1 => "your starting paranoia level. a difficulty slider, essentially.\nread more about paranoia in the manual.",
        2 => "the item which you wanna start with.\nnote that the items can be found in-game. this is but an \"initial edge\".",
        3 => "the perk you wanna start with. a double-edged sword... it cannot be changed, so choose wisely.",
        _ => "if you see this I screwed up. oops"
    };

    Paragraph::new(text).alignment(Alignment::Center)
}