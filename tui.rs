use crossterm::event::KeyCode;
use ratatui::DefaultTerminal;

use crate::{app::App, event_handler::handle_events, events::{EventHandler, FEvents}, ui::view_ui};

pub fn tui_run(terminal: &mut DefaultTerminal, app: &mut App) -> anyhow::Result<()> {
    let event = EventHandler::new(250);
   
    loop {
        terminal.draw(|f| {view_ui(f, app);})?;
        
        if let FEvents::Input(key) = event.next()? {
            handle_events(key, app)?;

            if key.code == KeyCode::Char('q') {
                break;
            }
        }
    }

    Ok(())
}
