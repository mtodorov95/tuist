#[derive(Debug)]
pub enum Screen {
    Exit,
    Main,
    Edit,
}

impl Default for Screen {
    fn default() -> Self {
        Self::Main
    }
}

#[derive(Debug, Default)]
pub struct Tab {
    pub url: String,
    pub content: String,
}

#[derive(Debug, Default)]
pub struct Browser {
    pub tabs: Vec<Tab>,
    pub active_tab: usize,
    pub scroll: u16,
    pub should_exit: bool,
    pub current_screen: Screen,
    pub currently_typing: bool,
}

impl Browser {
    pub fn new() -> Self {
        let tab = Tab::default();
        Self {
            tabs: vec![tab],
            active_tab: 0,
            scroll: 0,
            should_exit: false,
            current_screen: Screen::Main,
            currently_typing: false,
        }
    }

    pub fn tick(&self) {
        todo!()
    }

    pub fn toggle_typing(&mut self) {
        self.currently_typing = !self.currently_typing;
    }

    pub fn quit(&mut self) {
        self.should_exit = true;
    }

    pub fn active_tab_mut(&mut self) -> &mut Tab {
        self.tabs.get_mut(self.active_tab).unwrap()
    }

    pub fn active_tab(&self) -> &Tab {
        self.tabs.get(self.active_tab).unwrap()
    }

    pub fn set_content(&mut self, value: String) {
        self.active_tab_mut().content = value;
    }

    pub fn has_content(&self) -> bool {
        !self.active_tab().content.is_empty()
    }

    pub fn set_url(&mut self, value: String) {
        self.active_tab_mut().url = value;
    }

    pub fn scroll_up(&mut self) {
        if self.scroll.checked_sub(2).is_some() {
            self.scroll -= 2;
        }
    }

    pub fn scroll_down(&mut self) {
        if self.scroll.checked_add(2).is_some() {
            let scroll = std::cmp::min(
                self.scroll + 2,
                self.active_tab()
                    .content
                    .lines()
                    .count()
                    .try_into()
                    .unwrap(),
            );
            self.scroll = scroll;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sets_browser_content_correctly() {
        let mut browser = Browser::default();
        browser.set_content("hello".into());
        assert_eq!(browser.active_tab().content, "hello".to_string());
    }
}
