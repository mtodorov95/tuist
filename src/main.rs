use anyhow::Result;
use browser::Browser;
use event::{Event, EventHandler};
use ratatui::{backend::CrosstermBackend, Terminal};
use tui::Tui;
use update::update;

mod browser;
mod engine;
mod event;
mod network;
mod tui;
mod ui;
mod update;

fn main() -> Result<()> {
    let mut browser = Browser::new();

    let backend = CrosstermBackend::new(std::io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);

    tui.enter()?;

    while !browser.should_exit {
        tui.draw(&mut browser)?;

        match tui.events.next()? {
            Event::Tick => {}
            Event::Key(key_event) => update(&mut browser, key_event),
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        };
    }

    tui.exit()?;
    Ok(())
}
