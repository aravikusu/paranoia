use ratatui::{Frame, layout::{Alignment, Constraint, Direction, Layout, Rect}, style::{Style, Styled, Stylize}, text::{Line, Text}, widgets::{Gauge, ListItem, Paragraph}};

use crate::{app::App, data::database::Describable, util::{block_preset, list_preset}};

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
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Fill(3),
                Constraint::Fill(2),
            ]
        )
        .split(main_layout);
    game_screen_half(app, inner[0], frame);
    info_screen_half(app, inner[1], frame);
}

pub fn game_screen_half(_app: &App, area: Rect, frame: &mut Frame) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Fill(3),
                Constraint::Fill(1),
            ]
        )
        .split(area);
    let scene_block = block_preset("scene")
    .title_bottom(Line::from("AREA NAME").right_aligned());

    let scene = Paragraph::new("REPLACE PARAGRAPH WITH SCENE").alignment(Alignment::Center).block(scene_block);
    frame.render_widget(scene, layout[0]);
    
    let info_block = block_preset("info");
    let info = Paragraph::new("> This is placeholder info text. I'm sure it'll say lots of cool stuff at some point.").block(info_block);
    frame.render_widget(info, layout[1]);
}

pub fn info_screen_half(app: &App, area: Rect, frame: &mut Frame) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Fill(2),
                Constraint::Fill(1),
                Constraint::Fill(2),
            ]
        )
        .split(area);

    let map_block = block_preset("local map");
    let map = Paragraph::new("[map placeholder]")
        .alignment(Alignment::Center)
        .block(map_block);
    frame.render_widget(map, layout[0]);
    
    stats_block(app, layout[1], frame);

    let list_items: Vec<ListItem> = app.main_game_state.player.inventory.iter()
        .map(|i| ListItem::new(Text::from(i.name()).alignment(Alignment::Center)))
        .collect();

    let inv_block = block_preset("inventory");
    let inv_list = list_preset(list_items, inv_block, app.settings.theme);

    frame.render_widget(inv_list, layout[2]);
}

fn stats_block(app: &App, area: Rect, frame: &mut Frame) {
    let stats_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(area);

    let left_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Fill(1),
        ])
        .split(stats_layout[0]);
    
    let cur_hp = app.main_game_state.player.character.cur_hp;
    let max_hp = app.main_game_state.player.character.max_hp;
    let hp_block = block_preset("hp")
        .title_bottom(Line::from(format!("{}/{}", cur_hp, max_hp)).right_aligned());
    let hp = Gauge::default()
        .block(hp_block)
        .percent(100)
        .label("")
        .gauge_style(Style::default().fg(app.settings.theme.fg_color()));
    frame.render_widget(hp, left_layout[0]);

    let paranoia_block = block_preset("paranoia")
        .title_bottom(Line::from(format!("{}%", app.main_game_state.player.paranoia)).right_aligned());
    let paranoia = Gauge::default()
        .block(paranoia_block)
        .percent(app.main_game_state.player.paranoia as u16)
        .label("")
        .gauge_style(Style::default().fg(app.settings.theme.fg_color()));
    frame.render_widget(paranoia, left_layout[1]);

    let stats_text_block = block_preset("stats");
    let stats_text = Paragraph::new(format!(
            "STR: {} INT: {} AGI: {} LUK: {}",
            app.main_game_state.player.character.stats.str,
            app.main_game_state.player.character.stats.int,
            app.main_game_state.player.character.stats.agi,
            app.main_game_state.player.character.stats.luk
        ))
        .alignment(Alignment::Center)
        .block(stats_text_block);
    frame.render_widget(stats_text, left_layout[2]);

    let portrait_block = block_preset("you");
    let portrait = Paragraph::new("PROBABLY SOME PRETTY FACE HERE OR SOMETHING")
        .alignment(Alignment::Center)
        .block(portrait_block);
    frame.render_widget(portrait, stats_layout[1]);
}
