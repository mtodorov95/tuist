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
    pub url_field: String,
    pub content: String,
}

impl Tab {
    fn set_url(&mut self) {
        self.url = self.url_field.clone();
    }
}

#[derive(Debug)]
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

    pub fn new_tab(&mut self) {
        let new_tab = Tab::default();
        let index = self.tabs.len();
        self.tabs.push(new_tab);
        self.active_tab = index;
    }

    pub fn close_active_tab(&mut self) {
        match self.tabs.len() {
            1 => self.current_screen = Screen::Exit,
            length => {
                if self.active_tab == length - 1 {
                    self.prev_tab();
                    self.tabs.remove(length - 1);
                } else {
                    self.tabs.remove(self.active_tab);
                }
            }
        }
    }

    pub fn next_tab(&mut self) {
        self.active_tab = std::cmp::min(self.active_tab + 1, self.tabs.len() - 1);
    }

    pub fn prev_tab(&mut self) {
        if self.active_tab.checked_sub(1).is_some() {
            self.active_tab -= 1;
        }
    }

    pub fn set_content(&mut self, value: String) {
        self.active_tab_mut().content = value;
    }

    pub fn has_content(&self) -> bool {
        !self.active_tab().content.is_empty()
    }

    pub fn set_url(&mut self) {
        self.active_tab_mut().set_url();
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
    fn instantiates_self_correctly() {
        let browser = Browser::new();
        assert_eq!(browser.active_tab, 0);
        assert_eq!(browser.tabs.len(), 1);
        assert_eq!(browser.active_tab().url_field, "".to_string());
        assert_eq!(browser.active_tab().url, "".to_string());
        assert_eq!(browser.active_tab().content, "".to_string());
    }

    #[test]
    fn can_not_scroll_below_zero() {
        let mut browser = Browser::new();
        assert_eq!(browser.scroll, 0);
        browser.scroll_down();
        assert_eq!(browser.scroll, 0);
    }

    #[test]
    fn sets_active_tab_content() {
        let mut browser = Browser::new();
        browser.set_content("Hello".to_string());
        assert_eq!(browser.active_tab().content, "Hello".to_string());
    }

    #[test]
    fn creates_new_tab() {
        let mut browser = Browser::new();
        browser.new_tab();
        assert_eq!(browser.active_tab, 1);
        assert_eq!(browser.tabs.len(), 2);
    }

    #[test]
    fn switches_to_next_tab() {
        let mut browser = Browser::new();
        browser.new_tab();
        browser.new_tab();
        assert_eq!(browser.tabs.len(), 3);
        browser.active_tab = 0;

        browser.next_tab();
        assert_eq!(browser.active_tab, 1);
        browser.next_tab();
        assert_eq!(browser.active_tab, 2);
        browser.next_tab();
        assert_eq!(browser.active_tab, 2);
    }

    #[test]
    fn switches_to_previous_tab() {
        let mut browser = Browser::new();
        browser.new_tab();
        browser.new_tab();
        assert_eq!(browser.tabs.len(), 3);

        assert_eq!(browser.active_tab, 2);
        browser.prev_tab();
        assert_eq!(browser.active_tab, 1);
        browser.prev_tab();
        assert_eq!(browser.active_tab, 0);
        browser.prev_tab();
        assert_eq!(browser.active_tab, 0);
    }

    #[test]
    fn closes_non_last_tab_correctly() {
        let mut browser = Browser::new();
        browser.new_tab();
        browser.new_tab();
        browser.new_tab();
        assert_eq!(browser.active_tab, 3);

        browser.active_tab = 1;
        browser.close_active_tab();
        assert_eq!(browser.tabs.len(), 3);
        assert_eq!(browser.active_tab, 1);
    }

    #[test]
    fn closes_last_tab_correctly() {
        let mut browser = Browser::new();
        browser.new_tab();
        browser.new_tab();
        browser.new_tab();
        assert_eq!(browser.active_tab, 3);

        browser.close_active_tab();
        assert_eq!(browser.tabs.len(), 3);
        assert_eq!(browser.active_tab, 2);
    }
}
