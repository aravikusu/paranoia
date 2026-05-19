use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::prelude::{Alignment, Line, Stylize};
use ratatui::style::{Style, Styled};
use ratatui::widgets::Paragraph;

use crate::app::App;
use crate::util::{block_preset, menu_header};

pub fn instructions(app: &App) -> Line<'static> {
    match app.game_setup_state.cursor.selected() {
        0 | 2 | 3 => {
            Line::from(vec![
                " navigate ".into(),
                "<W/A/UP/DOWN> ".set_style(Style::default().fg(app.settings.theme.highlight_color())).bold(),
                if app.game_setup_state.editing_name { " deselect ".into() } else { " select ".into() },
                "<ENTER> ".set_style(Style::default().fg(app.settings.theme.highlight_color())).bold(),
                if app.game_setup_state.editing_name { " deselect ".into() } else { " go back ".into() },
                "<ESC> ".set_style(Style::default().fg(app.settings.theme.highlight_color())).bold(),
            ])
        }
        _ => Line::from(vec![
            " navigate ".into(),
            "<W/A/UP/DOWN> ".set_style(Style::default().fg(app.settings.theme.highlight_color())).bold(),
            " increment/decrement ".into(),
            "<A/D/LEFT/RIGHT> ".set_style(Style::default().fg(app.settings.theme.highlight_color())).bold(),
            " go back ".into(),
            "<ESC> ".set_style(Style::default().fg(app.settings.theme.highlight_color())).bold(),
        ])
    }
}

pub fn layout(app: &App, frame: &mut Frame, main_block: Rect) {
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
        .split(main_block);

    let header = vec![
        "-game setup-".bold().into(),
        "setup your game.".italic().into(),
    ];
    frame.render_widget(
        menu_header(header, app.settings.theme.fg_color()),
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

    let name_block = block_preset("name");
    frame.render_widget(
        menu_block_text(
            app.game_setup_state.name.clone(),
            app,
            0,
        ).block(name_block), settings_layout[0]);

    let paranoia_block = block_preset("paranoia level")
        .title_bottom(Line::from(app.game_setup_state.paranoia.to_string()).right_aligned());

    let paranoia = app.game_setup_state.paranoia.clamp(0, 100) as usize;
    let bar_width = 28;
    let filled = (paranoia * bar_width) / 100;
    let bar = "█".repeat(filled) + &"░".repeat(bar_width - filled);

    frame.render_widget(
        menu_block_text(
            bar,
            app,
            1,
        ).block(paranoia_block), settings_layout[1]);

    let item_block = block_preset("start item");
    frame.render_widget(
        menu_block_text(
            "".into(),
            app,
            2,
        ).block(item_block), settings_layout[2]);

    let perk_block = block_preset("start perk");
    frame.render_widget(
        menu_block_text(
            "".into(),
            app,
            3,
        ).block(perk_block), settings_layout[3]);

    let info_block = block_preset("info box");
    frame.render_widget(info_block_text(app).alignment(Alignment::Center).block(info_block), inner[5]);
}

fn menu_block_text<'a>(text: String, app: &App, menu_idx: usize) -> Paragraph<'a> {
    let mut paragraph = Paragraph::new(text)
        .alignment(Alignment::Left);

    if app.game_setup_state.cursor.selected() == menu_idx {
        paragraph = paragraph
            .style(Style::default().bg(
                if app.game_setup_state.editing_name { app.settings.theme.highlight_color() } else { app.settings.theme.fg_color() }
            ))
            .black();
    }
    paragraph
}

fn info_block_text<'a>(app: &App) -> Paragraph<'a> {
    let text = match app.game_setup_state.cursor.selected() {
        0 => "your... name. what everyone will refer to you as\nyou know... a name..".to_string(),
        1 => {
            let mut thing = String::from("your starting paranoia level. a difficulty slider, essentially.\nread more about paranoia in the manual.");
            if app.game_setup_state.paranoia > 50 {
                thing.push_str("\n\nWARNING: PARANOIA AT 50 ALREADY PUTS YOU AT DANGER FROM THE GET-GO. GOING ABOVE IT MEANS RISKING IMPOSSIBLE RUNS AND INSTANT DEATH.\nYOU HAVE BEEN WARNED.");
            }
            thing
        },
        2 => "the item which you wanna start with.\nnote that the items can be found in-game. this is but an \"initial edge\".".to_string(),
        3 => "the perk you wanna start with. a double-edged sword... it cannot be changed, so choose wisely.".to_string(),
        _ => "if you see this I screwed up. oops".to_string(),
    };

    Paragraph::new(text).alignment(Alignment::Center)
}
