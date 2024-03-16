use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Padding, Paragraph, Tabs, Wrap},
    Frame,
};

use crate::browser::{Browser, Screen};

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
        .border_type(BorderType::Rounded)
        .padding(Padding::horizontal(2));

    let content = match browser.has_content() {
        false => Paragraph::new("New Tab")
            .block(content_block)
            .style(Style::default().fg(Color::Yellow))
            .alignment(Alignment::Center),
        true => Paragraph::new(&*browser.active_tab().content)
            .block(content_block)
            .style(Style::default().fg(Color::Yellow))
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: false })
            .scroll((browser.scroll, 0)),
    };

    f.render_widget(content, chunks[0]);

    // Tab bar
    let mut tab_items = Vec::<Line>::new();

    for tab in &browser.tabs {
        tab_items.push(Line::from(Span::styled(
            &tab.url,
            Style::default().fg(Color::Yellow),
        )));
    }

    let tabs = Tabs::new(tab_items)
        .block(
            Block::default()
                .title("Tabs")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::Yellow))
        .highlight_style(Style::default().fg(Color::Red))
        .select(browser.active_tab);

    f.render_widget(tabs, chunks[1]);

    // Edit screen
    if let Screen::Edit = browser.current_screen {
        let area = create_centered_rect(60, 5, f.size());
        let url_block = Block::default().title("Address").borders(Borders::ALL);

        let url_text = Paragraph::new(browser.active_tab().url_field.as_str()).block(url_block);
        f.render_widget(url_text, area);
    }

    // Exit screen
    if let Screen::Exit = browser.current_screen {
        let area = create_centered_rect(60, 5, f.size());
        let popup_block = Block::default()
            .title("Y/N")
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::Red));

        let exit_text = Text::styled("Are you sure you want to exit? (y/n)", Style::default());

        let exit_paragraph = Paragraph::new(exit_text)
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::Yellow))
            .block(popup_block);
        f.render_widget(exit_paragraph, area);
    }
}

fn create_centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(layout[1])[1]
}
