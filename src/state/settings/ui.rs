use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, ListItem, ListState};
use ratatui::style::Styled;
use crate::app::App;
use crate::app_settings::AppTheme;
use crate::util::{self, menu_header};

pub fn instructions(app: &App) -> Line<'static> {
    Line::from(vec![
        " toggle ".into(),
        "<ENTER> ".set_style(Style::default().fg(AppTheme::highlight_color(&app.settings.theme))).bold(),
        " navigate ".into(),
        "<WASD/ARROW KEYS> ".set_style(Style::default().fg(AppTheme::highlight_color(&app.settings.theme))).bold(),
        " go back ".into(),
        "<ESC/BACKSPACE> ".set_style(Style::default().fg(AppTheme::highlight_color(&app.settings.theme))).bold(),
    ])
}

pub fn layout(app: &App, frame: &mut Frame, main_layout: [Rect; 1]) {
    let inner = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(2),
            Constraint::Percentage(10),
            Constraint::Percentage(5),
            Constraint::Fill(1),
            Constraint::Percentage(10),
        ])
        .split(main_layout[0]);


    let header = vec![
        "-paranoia settings-".bold().into(),
        "customize your experience.".italic().into(),
    ];
    frame.render_widget(
        menu_header(header, AppTheme::fg_color(&app.settings.theme)),
        inner[1]);

    let main_part = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(35),
            Constraint::Percentage(30),
            Constraint::Percentage(35),
        ])
        .split(inner[3]);

    let page = Line::from(vec![format!("page {}/1", &app.settings_state.page_idx).into()]);
    let page_name = get_page_name(&app.settings_state.page_idx);

    let list_block = Block::default()
        .title(page_name)
        .title_bottom(page.right_aligned())
        .title_alignment(Alignment::Left)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(AppTheme::fg_color(&app.settings.theme)));

    let page1items = [
        ListItem::new(Text::from("default").alignment(Alignment::Center)),
        ListItem::new(Text::from("vaporwave").alignment(Alignment::Center)),
        ListItem::new(Text::from("piccolo").alignment(Alignment::Center)),
        ListItem::new(Text::from("monochrome").alignment(Alignment::Center)),
    ];

    let items = match app.settings_state.page_idx {
        1 => page1items,
        _ => page1items
    };

    let settings = util::list_preset(items, list_block, app.settings.theme);

    frame.render_stateful_widget(
        settings,
        main_part[1],
        &mut ListState::default().with_selected(Some(app.settings_state.cursor.selected())));
}

fn get_page_name(idx: &usize) -> Line<'_> {
    match idx {
        1 => Line::from("themes"),
        _ => Line::from("themes"),
    }
}
