use ratatui::{
    Frame, 
    layout::{
        Constraint, Direction, Layout
    }, 
    style::{
        Color, Style, Stylize
    }, 
    text::{
        Line, Span, Text
    }, 
    widgets::{
        Block, BorderType, Borders, Clear, HighlightSpacing, List, ListItem, Padding, Paragraph, StatefulWidget, Widget
    }
};
use tui_big_text::{BigText, PixelSize};
use crate::app::{
    Colors as Game_Colors, Scene, Simon
};

pub fn ui(frame: &mut Frame, simon: &mut Simon) {
    match &simon.app_state.current_scene {
        Scene::MainMenu => { draw_main_menu(frame, simon); }
        Scene::Game => { draw_game(frame, simon); }
        Scene::Scores => { draw_score(frame, simon);}
        _ => {}
    }

    if simon.app_state.is_paused {
        draw_scene_modal(frame, simon);
    }

    
}

fn draw_scene_modal(frame: &mut Frame, simon: &mut Simon) {
    let menu_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());


    let menu_items: Vec<ListItem> = simon.app_state.menu_list.items
            .iter()
            .enumerate()
            .map(|(_i, menu_item)| {
                //let color = alternate_colors(i);
                ListItem::from(menu_item.name)//.bg(color)
            })
            .collect();
    let menu_list = List::new(menu_items)
        .block(menu_block)
        .highlight_symbol(">")
        .highlight_spacing(HighlightSpacing::Always);

    draw_stateful_center_modal(frame, menu_list, &mut simon.app_state.menu_list.state);
}

fn draw_stateful_center_modal<W, S>(frame: &mut Frame, widget: W, state: &mut S)
    where W: StatefulWidget<State = S> {
    // No behind bleed-through, no need for background
    frame.render_widget(Clear, frame.area().centered(
        Constraint::Percentage(40), 
        Constraint::Percentage(40)));

    frame.render_stateful_widget(widget, frame.area().centered(
        Constraint::Percentage(40), 
        Constraint::Percentage(40)),
        state
    );
}

fn draw_center_modal<W: Widget>(frame: &mut Frame, widget: W) {
    // No behind bleed-through, no need for background
    frame.render_widget(Clear, frame.area().centered(
        Constraint::Percentage(40), 
        Constraint::Percentage(40)));

    frame.render_widget(widget, frame.area().centered(
        Constraint::Percentage(40), 
        Constraint::Percentage(40)));
}

fn draw_main_menu(frame: &mut Frame, simon: &mut Simon) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(6),
            Constraint::Min(6)
        ])
        .split(frame.area());

    let says_line = Line::from(vec![
        Span::raw("Simon "),
        "S".red().bold(),
        "A".yellow().bold(),
        "Y".green().bold(),
        "S".blue().bold(),
    ]);

    /*  Would like to do this but currently always selects HalfWidth
        not critical, return to this later
    
    let pixel_size: PixelSize;
    if frame.area().x > 5 {
        pixel_size = PixelSize::Full
    } else {
        pixel_size = PixelSize::HalfWidth
    }; */
    
    let big_title = BigText::builder()
        .pixel_size(PixelSize::Full)
        .lines(vec![
            says_line
        ])
        .centered()
        .build();

    // Todo: Add list with items to select Play / Scores

    frame.render_widget(big_title, chunks[0]);

    let menu_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());


    let menu_items: Vec<ListItem> = simon.app_state.menu_list.items
            .iter()
            .enumerate()
            .map(|(_i, menu_item)| {
                //let color = alternate_colors(i);
                ListItem::from(menu_item.name)//.bg(color)
            })
            .collect();
    let menu_list = List::new(menu_items)
        .block(menu_block)
        .highlight_symbol(">")
        .highlight_spacing(HighlightSpacing::Always);

    frame.render_stateful_widget(
        menu_list, 
        chunks[1].centered(Constraint::Percentage(50), Constraint::Percentage(50)), 
        &mut simon.app_state.menu_list.state
    );
    //frame.render_widget(menu_list, chunks[1].centered(Constraint::Percentage(50), Constraint::Percentage(50)));
}

fn draw_game(frame: &mut Frame, simon: &mut Simon) {
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

    let buttons_block = Block::default()
        .borders(Borders::ALL)
        .padding(Padding::new(1, 1, 1, 1))
        .style(Style::default());
    // clone first
    let button_block_area = buttons_block.inner(chunks[1]).clone();
    // move value later
    frame.render_widget(buttons_block, chunks[1]);

    let button_row_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50)
        ])
        .split(button_block_area);

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

    let normal_border= Borders::NONE;
    let selected_border = Borders::ALL;

    let mut red_border = normal_border;
    let mut yellow_border = normal_border;
    let mut green_border = normal_border;
    let mut blue_border = normal_border;

    let mut red_color: ratatui::style::Color = Color::Red;
    let mut yellow_color: ratatui::style::Color = Color::Yellow;
    let mut green_color: ratatui::style::Color = Color::Green;
    let mut blue_color: ratatui::style::Color = Color::Blue;
    match simon.game_state.shown_color {
        Some(Game_Colors::RED) => {
            red_color = Color::LightRed;
            red_border = selected_border;
        },
        Some(Game_Colors::YELLOW) => {
            yellow_color = Color::LightYellow;
            yellow_border = selected_border;
        },
        Some(Game_Colors::GREEN) => {
            green_color = Color::LightGreen;
            green_border = selected_border;
        },
        Some(Game_Colors::BLUE) => {
            blue_color = Color::LightBlue;
            blue_border = selected_border;
        },
        None => {}
    }

    let red_block = Block::default()
            .borders(red_border)
            .border_type(BorderType::Thick)
            .padding(Padding::new(5, 5, 5, 5))
            .style(Style::default().bg(red_color));
    
    frame.render_widget(red_block, button_top_chunks[0]);

    let yellow_block = Block::default()
            .borders(yellow_border)
            .border_type(BorderType::Thick)
            .padding(Padding::new(5, 5, 5, 5))
            .style(Style::default().bg(yellow_color));
    
    frame.render_widget(yellow_block, button_top_chunks[1]);

    let green_block = Block::default()
            .borders(green_border)
            .border_type(BorderType::Thick)
            .padding(Padding::new(5, 5, 5, 5))
            .style(Style::default().bg(green_color));
    
    frame.render_widget(green_block, button_bottom_chunks[0]);

    let blue_block = Block::default()
            .borders(blue_border)
            .border_type(BorderType::Thick)
            .padding(Padding::new(5, 5, 5, 5))
            .style(Style::default().bg(blue_color));
    
    
    frame.render_widget(blue_block, button_bottom_chunks[1]);


    simon.game_state.clickables.clear();

    simon.game_state.clickables.push((Game_Colors::RED, button_top_chunks[0]));
    simon.game_state.clickables.push((Game_Colors::YELLOW, button_top_chunks[1]));
    simon.game_state.clickables.push((Game_Colors::GREEN, button_bottom_chunks[0]));
    simon.game_state.clickables.push((Game_Colors::BLUE, button_bottom_chunks[1]));

    
    /* DEBUG TITLE */
    let debug_msg = format!("{}", simon.debug_msg);

    let dubug_block = Block::default()
        .borders(Borders::ALL);

    let debug_paragraph = Paragraph::new(Text::styled(
        debug_msg,
        Style::default().fg(Color::White)
    )).block(dubug_block);

    frame.render_widget(debug_paragraph, title_chunks[1]);

    /* TODO - REMOVE DEBUG TITLE */


}

fn draw_score(frame: &mut Frame, simon: &mut Simon) {
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
        "Simon's Scores",
        Style::default().fg(Color::White)
    )).block(title_block);

    frame.render_widget(title, title_chunks[0]);

    let scores_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let score_items: Vec<ListItem> = simon.score_state.score_list.scores
        .iter()
            .enumerate()
            .map(|(_i, score)| {
                //let color = alternate_colors(i);
                ListItem::from(format!("{}  -  {}", score.name, score.score))//.bg(color)
            })
            .collect();

    let scores_list = List::new(score_items)
        .block(scores_block)
        .highlight_symbol(">")
        .highlight_spacing(HighlightSpacing::Always);


    frame.render_stateful_widget(
        scores_list, 
        chunks[1], 
        &mut simon.score_state.score_list.state
    );

}