use paranoia::app::{App, AppResult};
use paranoia::event::{Event, EventHandler};
use paranoia::tui::Tui;
use crossterm::event::{KeyCode, KeyModifiers};
use std::io;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

fn main() -> AppResult<()> {
    // Create an application.
    let mut app = App::new();

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    // Start the main loop.
    while app.running {
        // Render the user interface.
        tui.draw(&mut app)?;
        // Handle events.
        match tui.events.next()? {
            Event::Tick => app.tick(),
            Event::Key(key_event) => {
                if key_event.code == KeyCode::Char('c') && key_event.modifiers.contains(KeyModifiers::CONTROL) {
                    app.quit();
                } else {
                    app.screen.handle_input(key_event, &mut app);
                }
            }
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        }
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
