use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::{browser::Browser, network};

pub fn update(browser: &mut Browser, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Up | KeyCode::Char('j') => browser.scroll_down(),
        KeyCode::Down | KeyCode::Char('k') => browser.scroll_up(),
        KeyCode::Enter => {
            // Hardcoded for now
            let url = "https://mariotodorov.com";
            let response = network::request(url.to_string()).unwrap();
            // let root = crate::engine::parse(response);
            let html = crate::engine::simple_parse(response);
            browser.set_content(html);
            browser.set_url(url.to_string());
        }
        KeyCode::Esc | KeyCode::Char('q') => browser.quit(),
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                browser.quit();
            }
        }
        _ => {}
    }
}
