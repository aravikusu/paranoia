use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, HighlightSpacing, List, ListDirection, ListItem, ListState, Paragraph};
use ratatui::widgets::block::{Position, Title};
use crate::app::App;
use crate::app_settings::AppTheme;

pub fn instructions(app: &App) -> Title<'static> {
    Title::from(Line::from(vec![
        " toggle ".into(),
        "<ENTER> ".set_style(Style::default().fg(AppTheme::highlight_color(&app.settings.theme))).bold(),
        " navigate ".into(),
        "<WASD/ARROW KEYS> ".set_style(Style::default().fg(AppTheme::highlight_color(&app.settings.theme))).bold(),
        " go back ".into(),
        "<ESC/BACKSPACE> ".set_style(Style::default().fg(AppTheme::highlight_color(&app.settings.theme))).bold(),
    ]))
}

pub fn layout(app: &mut App, frame: &mut Frame, main_block: Block) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(100)
            ]
        )
        .split(main_block.inner(frame.size()));
    frame.render_widget(main_block, frame.size());

    let inner = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(2),
            Constraint::Percentage(10),
            Constraint::Percentage(5),
            Constraint::Fill(1),
            Constraint::Percentage(10),
        ])
        .split(chunks[0]);


    let text = vec![
        "-paranoia settings-".bold().into(),
        "customize your experience.".italic().into(),
    ];
    let header = Paragraph::new(text);
    frame.render_widget(
        header.alignment(Alignment::Center).style(Style::default().fg(AppTheme::fg_color(&app.settings.theme))),
        inner[1],
    );

    let main_part = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(35),
            Constraint::Percentage(30),
            Constraint::Percentage(35),
        ])
        .split(inner[3]);

    let page = Title::from(Line::from(vec![format!("page{}/1", &app.settings_state.page_idx).into()]));
    let page_name = get_page_name(&app.settings_state.page_idx);

    let list_block = Block::default()
        .title("settings")
        .title(
            page
                .alignment(Alignment::Right)
                .position(Position::Bottom),
        )
        .title(
            page_name
                .alignment(Alignment::Left)
                .position(Position::Bottom),
        )
        .title_alignment(Alignment::Left)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(AppTheme::fg_color(&app.settings.theme)));

    let page1items = [
        ListItem::new(Text::from("default").alignment(Alignment::Center)),
        ListItem::new(Text::from("vaporwave").alignment(Alignment::Center)),
    ];

    let items = match app.settings_state.page_idx {
        1 => page1items,
        _ => page1items
    };

    let settings = List::new(items)
        .block(list_block)
        .style(Style::default().fg(AppTheme::fg_color(&app.settings.theme)))
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::ITALIC)
                .fg(Color::Black)
                .bg(AppTheme::fg_color(&app.settings.theme))
        )
        .highlight_symbol(">>")
        .repeat_highlight_symbol(true)
        .highlight_spacing(HighlightSpacing::Always)
        .direction(ListDirection::TopToBottom);

    frame.render_stateful_widget(
        settings,
        main_part[1],
        &mut ListState::default().with_selected(Some(app.settings_state.menu_idx)));
}

fn get_page_name(idx: &usize) -> Title {
    match idx {
        1 => Title::from("themes"),
        _ => Title::from("themes"),
    }
}