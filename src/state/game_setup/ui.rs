use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::prelude::{Alignment, Line, Stylize};
use ratatui::style::{Color, Style, Styled};
use ratatui::text::{Span, Text};
use ratatui::widgets::{Clear, Gauge, ListItem, ListState, Paragraph};

use crate::app::App;
use crate::data::database::Describable;
use crate::util::{block_preset, list_preset, menu_header};

pub fn instructions(app: &App) -> Line<'static> {
    if app.game_setup_state.editing_stats {
        return Line::from(vec![
            " navigate ".into(),
            "<A/D/LEFT/RIGHT> ".set_style(Style::default().fg(app.settings.theme.highlight_color())).bold(),
            if app.game_setup_state.editing_name { " deselect ".into() } else { " increment/decrement ".into() },
            "<W/A/UP/DOWN> ".set_style(Style::default().fg(app.settings.theme.highlight_color())).bold(),
            if app.game_setup_state.editing_name { " deselect ".into() } else { " go back ".into() },
            "<ENTER/ESC> ".set_style(Style::default().fg(app.settings.theme.highlight_color())).bold(),
        ])
    }
    match app.game_setup_state.cursor.selected() {
        0 | 1 | 3 | 4 | 5 => {
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
                Constraint::Length(21),
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
    
    let is_stats_selected = app.game_setup_state.cursor.selected() == 1;
    let stats = [
        ("STR", app.game_setup_state.stats.str),
        ("INT", app.game_setup_state.stats.int),
        ("AGI", app.game_setup_state.stats.agi),
        ("LUK", app.game_setup_state.stats.luk)
    ];

    let spans: Vec<Span> = stats.iter().enumerate().map(|(idx, (name, value))| {
        let style = if is_stats_selected && app.game_setup_state.editing_stats && app.game_setup_state.stats_cursor.selected() == idx {
            Style::default().bg(app.settings.theme.fg_color()).fg(Color::Black)
        } else {
            Style::default()
        };
        format!("{}:{} ", name, value).set_style(style)
    }).collect();

    let mut paragraph = Paragraph::new(Line::from(spans).alignment(Alignment::Center));
    if is_stats_selected && !app.game_setup_state.editing_stats {
        paragraph = paragraph
            .style(Style::default().bg(app.settings.theme.fg_color()))
            .black();
    }

    let stats_block = block_preset("stats")
        .style(
            if is_stats_selected && ! app.game_setup_state.editing_stats {
                Style::default().fg(Color::Black)
            } else {
                Style::default().fg(app.settings.theme.fg_color())
            }
        )
        .title_bottom(Line::from(format!("{} points left to allocate", app.game_setup_state.stat_points_to_allocate)).right_aligned());

    frame.render_widget(paragraph.block(stats_block), settings_layout[1]);

    let paranoia_block = block_preset("paranoia level")
        .title_bottom(Line::from(app.game_setup_state.paranoia.to_string()).right_aligned());
    
    let paranoia = app.game_setup_state.paranoia.clamp(0, 100) as u16;
    let guage = Gauge::default()
        .block(paranoia_block)
        .style(
            if app.game_setup_state.cursor.selected() == 2 {
                Style::default().bg(app.settings.theme.fg_color()).fg(Color::Black)
            } else {
                Style::default()
            }
        )
        .gauge_style(
            if app.game_setup_state.cursor.selected() == 2 {
                Style::default().bg(app.settings.theme.fg_color()).fg(Color::Black)
            } else {
                Style::default().fg(app.settings.theme.fg_color())
            }
        )
        .label("")
        .use_unicode(true)
        .percent(paranoia);
    frame.render_widget(guage, settings_layout[2]);
    
    let item_name = app.game_setup_state.item.as_ref().map_or(String::new(), |i| i.name.clone());
    let item_block = block_preset("start item");
    frame.render_widget(
        menu_block_text(
            item_name,
            app,
            3,
        ).block(item_block), settings_layout[3]);

    let perk_name = app.game_setup_state.perk.as_ref().map_or(String::new(), |i| i.name.clone());
    let perk_block = block_preset("perk");
    frame.render_widget(
        menu_block_text(
            perk_name,
            app,
            4,
        ).block(perk_block), settings_layout[4]);

    let start_block = block_preset("");
    frame.render_widget(
        start_button(
            app,
            5,
        ).block(start_block), settings_layout[5]);

    let info_block = block_preset("info box");
    frame.render_widget(info_block_text(app).alignment(Alignment::Center).block(info_block), inner[5]);
    
    if app.game_setup_state.selecting_item {
        let area = centered_rect(50, 60, main_block);
        render_dialog(app, frame, area, "select starting item", &app.game_setup_state.starting_items);
    }

    if app.game_setup_state.selecting_perk {
        let area = centered_rect(50, 60, main_block);
        render_dialog(app, frame, area, "select starting perk", &app.game_setup_state.perks);
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

fn render_dialog<T: Describable>(app: &App, frame: &mut Frame, area: Rect, title: &str, items: &[T]) {
    frame.render_widget(Clear, area);
    let block = block_preset(title);
    let list_items: Vec<ListItem> = items.iter()
        .map(|i| ListItem::new(Text::from(i.name()).alignment(Alignment::Center)))
        .collect();

    let list = list_preset(list_items, block, app.settings.theme);
    frame.render_stateful_widget(
        list,
        area,
        &mut ListState::default().with_selected(Some(app.game_setup_state.submenu_cursor.selected())),
    );
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

fn start_button<'a>(app: &App, menu_idx: usize) -> Paragraph<'a> {
    let mut paragraph = Paragraph::new("GAME START")
        .alignment(Alignment::Center);

    if app.game_setup_state.cursor.selected() == menu_idx {
        paragraph = paragraph
            .style(Style::default().bg(
                if !app.game_setup_state.start_available() {
                    app.settings.theme.disabled_color()
                }
                else {
                    if app.game_setup_state.editing_name {
                        app.settings.theme.highlight_color() 
                    }
                    else { 
                        app.settings.theme.fg_color() 
                    }
                }
                
            ))
            .black();
    }

    paragraph
}

fn info_block_text<'a>(app: &App) -> Paragraph<'a> {
    if app.game_setup_state.selecting_item {
        let idx = app.game_setup_state.submenu_cursor.selected();
        let desc = app.game_setup_state.starting_items.get(idx).map(|i| i.description()).unwrap_or("");
        return Paragraph::new(desc.to_string()).alignment(Alignment::Center);
    }

    if app.game_setup_state.selecting_perk {
        let idx = app.game_setup_state.submenu_cursor.selected();
        let desc = app.game_setup_state.perks.get(idx).map(|p| p.description()).unwrap_or("");
        return Paragraph::new(desc.to_string()).alignment(Alignment::Center);
    }

    let text = match app.game_setup_state.cursor.selected() {
        0 => "your... name. what everyone will refer to you as\nyou know... a name..".to_string(),
        1 => "your stats. they let you do things, or something like that.".to_string(),
        2 => {
            let mut thing = String::from("your starting paranoia level. the more of it you have, the more things may be off...");
            if app.game_setup_state.paranoia > 50 {
                thing.push_str("\n\nWARNING: PARANOIA AT 50 ALREADY PUTS YOU AT DANGER FROM THE GET-GO. GOING ABOVE IT MEANS RISKING IMPOSSIBLE RUNS AND INSTANT DEATH.\nYOU HAVE BEEN WARNED.");
            }
            thing
        },
        3 => "the item which you wanna start with.\nnote that most of these items can be obtained in-game.".to_string(),
        4 => "the perk you want to give your character. the perk you choose cannot be changed and greatly affects your playthrough.".to_string(),
        5 => {
            if app.game_setup_state.start_available() {
                "for better or worse, this starts the game.".to_string()
            }
            else {
                "in order to start the game you must have a name, allocate all of your stats, and choose a perk.".to_string()
            }
        },
        _ => "if you see this I screwed up. oops".to_string(),
    };
    Paragraph::new(text).alignment(Alignment::Center)
}
