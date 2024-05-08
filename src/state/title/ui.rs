use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, HighlightSpacing, List, ListDirection, ListItem, ListState, Paragraph};
use ratatui::widgets::block::Title;
use crate::app::App;

const LOGO: &str = "_________                                         .__    _____
\\______   \\_____   _______ _____     ____    ____  |__|  /  _  \\
  |     ___/\\__  \\  \\_  __ \\__   \\   /    \\  /  _ \\ |  | /  /_\\  \\
  |    |     / __ \\_ |  | \\/ / __ \\_|   |  \\(  <_> )|  |/    |    \\
  |____|    (____  / |__|   (____  /|___|  / \\____/ |__|\\____|__  /
                \\/              \\/      \\/                     \\/
-a text adventure by aravix-
";

pub fn instructions() -> Title<'static> {
    Title::from(Line::from(vec![
        " select ".into(),
        "<ENTER> ".light_red().bold(),
        " navigate ".into(),
        "<WASD/ARROW KEYS> ".light_red().bold(),
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
        .constraints(
            [
                Constraint::Percentage(10),
                Constraint::Percentage(50),
                Constraint::Percentage(5),
                Constraint::Percentage(35)
            ]
        )
        .split(chunks[0]);


    frame.render_widget(
        Paragraph::new(LOGO)
            .block(Block::new().borders(Borders::empty()))
            .style(Style::default().fg(Color::LightYellow).bg(Color::Black))
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
                .border_style(Style::default().fg(Color::LightYellow))
        )
        .style(Style::default().fg(Color::LightYellow))
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::ITALIC)
                .fg(Color::Black)
                .bg(Color::LightYellow)
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