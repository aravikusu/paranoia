use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, HighlightSpacing, List, ListDirection, ListItem, ListState, Paragraph};
use ratatui::widgets::block::Title;
use crate::app::App;
use crate::app_settings::AppTheme;

const LOGO: &str = "_________                                         .__    _____
\\______   \\_____   _______ _____     ____    ____  |__|  /  _  \\
  |     ___/\\__  \\  \\_  __ \\__   \\   /    \\  /  _ \\ |  | /  /_\\  \\
  |    |     / __ \\_ |  | \\/ / __ \\_|   |  \\(  <_> )|  |/    |    \\
  |____|    (____  / |__|   (____  /|___|  / \\____/ |__|\\____|__  /
                \\/              \\/      \\/                     \\/
-a text adventure by aravix-
";

pub fn instructions(app: &App) -> Title<'static> {
    Title::from(Line::from(vec![
        " select ".into(),
        "<ENTER> ".set_style(Style::default().fg(AppTheme::highlight_color(&app.settings.theme))).bold(),
        " navigate ".into(),
        "<WASD/ARROW KEYS> ".set_style(Style::default().fg(AppTheme::highlight_color(&app.settings.theme))).bold(),
    ]))
}

pub fn layout(app: &mut App, frame: &mut Frame, main_layout: [Rect; 1]) {
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
        .split(main_layout[0]);


    frame.render_widget(
        Paragraph::new(LOGO)
            .block(Block::new().borders(Borders::empty()))
            .style(Style::default().fg(AppTheme::fg_color(&app.settings.theme)))
            .centered(),
        inner[1],
    );

    let menu_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(40),
                Constraint::Percentage(20),
                Constraint::Percentage(40)
            ]
        )
        .split(inner[3]);

    let items = [
        ListItem::new(Text::from("start game").alignment(Alignment::Center)),
        ListItem::new(Text::from("settings").alignment(Alignment::Center)),
        ListItem::new(Text::from("manual").alignment(Alignment::Center)),
        ListItem::new(Text::from("quit").alignment(Alignment::Center)),
    ];
    let options = List::new(items)
        .block(
            Block::default()
                .title("main menu")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(AppTheme::fg_color(&app.settings.theme)))
        )
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
        options,
        menu_layout[1],
        &mut ListState::default().with_selected(Some(app.title_state.menu_idx)));
    //frame.render_widget(options, inner[2]);
}