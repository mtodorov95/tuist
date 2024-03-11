use std::collections::HashMap;

use crate::engine::dom::elem;

use super::dom::{text, AttrMap, Node};

static SELF_CLOSING_TAGS: [&str; 14] = [
    "area", "base", "br", "col", "embed", "hr", "img", "input", "link", "meta", "param", "source",
    "track", "wbr",
];

struct Parser {
    pos: usize,
    input: String,
}

impl Parser {
    fn next_char(&self) -> char {
        self.input[self.pos..].chars().next().unwrap()
    }

    fn starts_with(&self, s: &str) -> bool {
        self.input[self.pos..].starts_with(s)
    }

    fn eof(&self) -> bool {
        self.pos >= self.input.len()
    }

    fn consume_char(&mut self) -> char {
        let mut iter = self.input[self.pos..].char_indices();
        let (_, curr) = iter.next().unwrap();
        let (next_pos, _) = iter.next().unwrap_or((1, ' '));
        self.pos += next_pos;
        curr
    }

    fn consume_while<F>(&mut self, test: F) -> String
    where
        F: Fn(char) -> bool,
    {
        let mut res = String::new();
        while !self.eof() && test(self.next_char()) {
            res.push(self.consume_char());
        }
        res
    }

    fn consume_whitespace(&mut self) {
        self.consume_while(|c: char| c.is_whitespace());
    }

    fn parse_tag_name(&mut self) -> String {
        self.consume_while(|c| {
            matches!(c,
            'a'..='z' | 'A'..='Z' | '0'..='9')
        })
    }

    fn parse_attr_name(&mut self) -> String {
        self.consume_while(|c| {
            matches!(c,
            'a'..='z' | 'A'..='Z' | '0'..='9' | ':' | '-')
        })
    }

    fn parse_node(&mut self) -> Option<Node> {
        match self.next_char() {
            '<' => self.parse_element(),
            _ => self.parse_text(),
        }
    }

    fn parse_nodes(&mut self) -> Vec<Node> {
        let mut nodes = Vec::new();
        loop {
            self.consume_whitespace();
            if self.eof() || self.starts_with("</") {
                break;
            }
            if let Some(node) = self.parse_node() {
                nodes.push(node);
            }
        }
        nodes
    }

    fn parse_text(&mut self) -> Option<Node> {
        Some(text(self.consume_while(|c| c != '<')))
    }

    fn parse_element(&mut self) -> Option<Node> {
        assert!(self.consume_char() == '<');
        if self.next_char() == '!' {
            self.consume_while(|c| c != '>');
            self.consume_char();
            return None;
        }

        let tag_name = self.parse_tag_name();

        if tag_name == "script" {
            self.skip_element(&tag_name);
            return None;
        }

        let attrs = self.parse_attributes();

        if SELF_CLOSING_TAGS.contains(&tag_name.as_str()) {
            if self.next_char() == '/' {
                assert!(self.consume_char() == '/');
            }
            assert!(self.consume_char() == '>');
            return Some(elem(tag_name, attrs, vec![]));
        }

        assert!(self.consume_char() == '>');
        let children = self.parse_nodes();
        assert!(self.consume_char() == '<');
        assert!(self.consume_char() == '/');
        assert!(self.parse_tag_name() == tag_name);
        assert!(self.consume_char() == '>');
        Some(elem(tag_name, attrs, children))
    }

    fn skip_element(&mut self, tag_name: &str) {
        loop {
            if self.starts_with(&format!("</{}", tag_name)) {
                break;
            }
            self.consume_char();
        }
        assert!(self.consume_char() == '<');
        assert!(self.consume_char() == '/');
        assert!(self.parse_tag_name() == tag_name);
        assert!(self.consume_char() == '>');
    }

    fn parse_attributes(&mut self) -> AttrMap {
        let mut attrs = HashMap::new();
        loop {
            self.consume_whitespace();
            if self.next_char() == '>' || self.next_char() == '/' {
                break;
            }

            let (name, value) = self.parse_attr();
            attrs.insert(name, value);
        }
        attrs
    }

    fn parse_attr(&mut self) -> (String, String) {
        let name = self.parse_attr_name();
        let value = match self.consume_char() {
            '=' => self.parse_attr_value(),
            _ => "".into(),
        };
        (name, value)
    }

    fn parse_attr_value(&mut self) -> String {
        if self.next_char() == '\\' {
            self.consume_char();
        }
        let open_quote = self.consume_char();
        assert!(open_quote == '"' || open_quote == '\'');
        let value = self.consume_while(|c| c != open_quote);
        assert!(self.consume_char() == open_quote);
        value
    }
}

pub fn parse(source: String) -> Node {
    let mut nodes = Parser {
        pos: 0,
        input: source,
    }
    .parse_nodes();
    if nodes.len() == 1 {
        nodes.swap_remove(0)
    } else {
        elem("html".to_string(), HashMap::new(), nodes)
    }
}
