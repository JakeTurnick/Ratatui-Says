use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap, Padding},
    Frame,
};

use crate::app::{Simon, Colors};

pub fn ui(frame: &mut Frame, simon: &Simon) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(6)
        ])
        .split(frame.area());

    let title_block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default());

    let title = Paragraph::new(Text::styled(
        "Simon Says",
        Style::default().fg(Color::White)
    )).block(title_block);

    frame.render_widget(title, chunks[0]);

    let button_row_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50)
        ])
        .split(chunks[1]);

    let button_top_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50)
        ])
        .split(button_row_chunks[0]);

    let button_bottom_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50)
        ])
        .split(button_row_chunks[1]);

    let red_block = Block::default()
            .borders(Borders::ALL)
            .padding(Padding::new(5, 5, 5, 5))
            .style(Style::default().bg(Color::Red));
    frame.render_widget(red_block, button_top_chunks[0]);

    let yellow_block = Block::default()
            .borders(Borders::ALL)
            .padding(Padding::new(5, 5, 5, 5))
            .style(Style::default().bg(Color::Yellow));
    frame.render_widget(yellow_block, button_top_chunks[1]);

    let green_block = Block::default()
            .borders(Borders::ALL)
            .padding(Padding::new(5, 5, 5, 5))
            .style(Style::default().bg(Color::Green));
    frame.render_widget(green_block, button_bottom_chunks[0]);

    let blue_block = Block::default()
            .borders(Borders::ALL)
            .padding(Padding::new(5, 5, 5, 5))
            .style(Style::default().bg(Color::Blue));
    frame.render_widget(blue_block, button_bottom_chunks[1]);


}