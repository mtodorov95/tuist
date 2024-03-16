use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::{
    browser::{Browser, Screen},
    engine::get_text_content,
    network,
};

pub fn update(browser: &mut Browser, key_event: KeyEvent) {
    match browser.current_screen {
        Screen::Main => match key_event.code {
            KeyCode::Char('j') => browser.scroll_down(),
            KeyCode::Char('k') => browser.scroll_up(),
            KeyCode::Char('p') => {
                if key_event.modifiers == KeyModifiers::CONTROL {
                    browser.current_screen = Screen::Edit;
                    browser.currently_typing = true;
                }
            }
            KeyCode::Esc | KeyCode::Char('q') => {
                browser.current_screen = Screen::Exit;
            }
            KeyCode::Char('c') | KeyCode::Char('C') => {
                if key_event.modifiers == KeyModifiers::CONTROL {
                    browser.current_screen = Screen::Exit;
                }
            }
            KeyCode::Char('t') | KeyCode::Char('T') => {
                if key_event.modifiers == KeyModifiers::CONTROL {
                    browser.new_tab();
                }
            }
            KeyCode::Tab => {
                if key_event.modifiers == KeyModifiers::ALT {
                    browser.prev_tab();
                } else {
                    browser.next_tab();
                }
            }
            _ => {}
        },
        Screen::Edit => match key_event.code {
            KeyCode::Enter => {
                if browser.currently_typing {
                    browser.set_url();
                    let response = network::request(&browser.active_tab().url).unwrap();
                    let root = crate::engine::parse(response);
                    let content = get_text_content(&root, &mut String::new());
                    browser.set_content(content);
                    browser.current_screen = Screen::Main;
                    browser.toggle_typing();
                }
            }
            KeyCode::Backspace => {
                if browser.currently_typing {
                    browser.active_tab_mut().url_field.pop();
                }
            }
            KeyCode::Esc => {
                browser.current_screen = Screen::Main;
                browser.toggle_typing();
            }
            KeyCode::Char(value) => {
                if browser.currently_typing {
                    browser.active_tab_mut().url_field.push(value);
                }
            }
            _ => {}
        },
        Screen::Exit => match key_event.code {
            KeyCode::Char('y') => browser.quit(),
            KeyCode::Char('n') | KeyCode::Char('q') => browser.current_screen = Screen::Main,
            _ => {}
        },
    }
}
