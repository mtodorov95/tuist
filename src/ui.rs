use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use crate::browser::Browser;

pub fn render(browser: &Browser, f: &mut Frame) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(3)])
        .split(f.size());

    // Content
    let content_block = Block::default()
        .title("Tuist")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    let content = Paragraph::new(browser.content.to_string())
        .block(content_block)
        .style(Style::default().fg(Color::Yellow))
        .alignment(Alignment::Left)
        .scroll((browser.scroll, 0));

    f.render_widget(content, chunks[0]);

    // Info
    let keys_hint = Span::styled(
        "Press 'Esc', 'Ctrl-C' or 'q' to exit.",
        Style::default().fg(Color::Red),
    );

    let keys_footer = Paragraph::new(Line::from(keys_hint)).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded),
    );

    let url = Span::styled(browser.url.to_string(), Style::default().fg(Color::Red));

    let url_footer = Paragraph::new(Line::from(url)).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded),
    );

    let footer_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[1]);

    f.render_widget(url_footer, footer_chunks[0]);
    f.render_widget(keys_footer, footer_chunks[1]);
}
