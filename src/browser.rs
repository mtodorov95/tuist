#[derive(Debug, Default)]
pub struct Browser {
    pub content: String,
    pub url: String,
    pub scroll: u16,
    pub should_exit: bool,
}

impl Browser {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn tick(&self) {}

    pub fn quit(&mut self) {
        self.should_exit = true;
    }

    pub fn set_content(&mut self, value: String) {
        self.content = value;
    }

    pub fn set_url(&mut self, value: String) {
        self.url = value;
    }

    pub fn scroll_up(&mut self) {
        if self.scroll.checked_sub(5).is_some() {
            self.scroll -= 5;
        }
    }

    pub fn scroll_down(&mut self) {
        if self.scroll.checked_add(5).is_some() {
            let scroll = std::cmp::min(
                self.scroll + 5,
                self.content.lines().count().try_into().unwrap(),
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
        assert_eq!(browser.content, "hello".to_string());
    }
}
