use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap, Padding},
    Frame,
};

use crate::app::{
    Simon, 
    Colors as Game_Colors,
};

pub fn ui(frame: &mut Frame, simon: &mut Simon) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(6)
        ])
        .split(frame.area());

    let title_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(25),
            Constraint::Percentage(75)
        ])
        .split(chunks[0]);

    let title_block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default());

    let title = Paragraph::new(Text::styled(
        "Simon Says",
        Style::default().fg(Color::White)
    )).block(title_block);

    frame.render_widget(title, title_chunks[0]);

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


    let mut red_color : ratatui::style::Color = Color::Red;
    let mut yellow_color : ratatui::style::Color = Color::Yellow;
    let mut green_color : ratatui::style::Color = Color::Green;
    let mut blue_color : ratatui::style::Color = Color::Blue;
    match simon.game_state.shown_color {
        Some(Game_Colors::RED) => red_color = Color::LightRed,
        Some(Game_Colors::YELLOW) => yellow_color = Color::LightYellow,
        Some(Game_Colors::GREEN) => green_color = Color::LightGreen,
        Some(Game_Colors::BLUE) => blue_color = Color::LightBlue,
        None => {}
    }

    let red_block = Block::default()
            .borders(Borders::ALL)
            .padding(Padding::new(5, 5, 5, 5))
            .style(Style::default().bg(red_color));
    let red_inner_area = red_block.inner(button_top_chunks[0]).clone();
    
    frame.render_widget(red_block, button_top_chunks[0]);

    let yellow_block = Block::default()
            .borders(Borders::ALL)
            .padding(Padding::new(5, 5, 5, 5))
            .style(Style::default().bg(yellow_color));
    let yellow_inner_area = yellow_block.inner(button_top_chunks[1]).clone();
    
    frame.render_widget(yellow_block, button_top_chunks[1]);

    let green_block = Block::default()
            .borders(Borders::ALL)
            .padding(Padding::new(5, 5, 5, 5))
            .style(Style::default().bg(green_color));
    let green_inner_area = green_block.inner(button_bottom_chunks[0]).clone();
    
    frame.render_widget(green_block, button_bottom_chunks[0]);

    let blue_block = Block::default()
            .borders(Borders::ALL)
            .padding(Padding::new(5, 5, 5, 5))
            .style(Style::default().bg(blue_color));
    
    let blue_inner_area = blue_block.inner(button_bottom_chunks[1]).clone();
    
    frame.render_widget(blue_block, button_bottom_chunks[1]);
    
    /*let button_bounds: Vec<Bounds_2d> = vec!(
        Bounds_2d {
            x_min: blue_inner_area.x,
            x_max: blue_inner_area.x + blue_inner_area.width,
            y_min: blue_inner_area.y, 
            y_max: blue_inner_area.y + blue_inner_area.height
        }
    );*/

    simon.game_state.clickables.clear();

    simon.game_state.clickables.push((Game_Colors::RED, red_inner_area));
    simon.game_state.clickables.push((Game_Colors::YELLOW, yellow_inner_area));
    simon.game_state.clickables.push((Game_Colors::GREEN, green_inner_area));
    simon.game_state.clickables.push((Game_Colors::BLUE, blue_inner_area));

    
    /* DEBUG TITLE */
    let debug_msg = format!("{}, frame time: {:?}", simon.debug_msg, simon.game_state.last_frame_time);

    let dubug_block = Block::default()
        .borders(Borders::ALL);

    let debug_paragraph = Paragraph::new(Text::styled(
        debug_msg,
        Style::default().fg(Color::White)
    )).block(dubug_block);

    frame.render_widget(debug_paragraph, title_chunks[1]);

    /* TODO - REMOVE DEBUG TITLE */


}