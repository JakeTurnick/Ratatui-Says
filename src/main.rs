use std::{
    error::Error, io, sync::mpsc, time::{Duration, Instant}, mem
};

use ratatui::{
    Terminal, backend::{
        Backend,
        CrosstermBackend
    }, crossterm::{
        event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, MouseButton},
        execute,
        terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode}
    } 
    //prelude::CrosstermBackend
};

mod app;
mod ui;
use crate::{
    app::{
        GameEvent, GameMode, Scene, Simon
    },
    ui::ui
};


fn main() -> Result<(), Box<dyn Error>> {
    // Set up terminal
    enable_raw_mode()?;
    let mut stderr = io::stderr();
     let _ = execute!(stderr, EnableMouseCapture, EnterAlternateScreen);

    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    // Create & Start simon App
    let mut simon = Simon::new();
    //simon.add_to_pattern(4); // Old way: method
    // new way: Associated function only borrows the current pattern, allowing the rest of the simon instance to be borrowed
    Simon::add_to_pattern(&mut simon.current_pattern, 4);

    let res = run_app(&mut terminal, &mut simon);

    // Clean up terminal
    // `?` pass errors back up to Box<dyn Error>
    disable_raw_mode()?;
    let _ = execute!(terminal.backend_mut(), DisableMouseCapture, LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Ok(_exit_game) = res {
        println!("\rThanks for playing!");
    } else if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, simon: &mut Simon) -> io::Result<()> {
    let (tx, rx) = mpsc::channel();
    let tick_rate = Duration::from_millis(16);

    // Input loop (event polling in separate thread to avoid blocking UI draws)
    let event_tx = tx.clone();
    std::thread::spawn(move || {
        let mut last_tick = Instant::now();
        loop {
            // define game ticks
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or(Duration::from_millis(1));

            // check for event within tick, sent to rx thread
            if event::poll(timeout).expect("Poll failed") {
                if let Ok(ev) = event::read() {
                    event_tx.send(GameEvent::Input(ev)).unwrap();
                }
            }

            // reset current tick
            if last_tick.elapsed() >= tick_rate {
                if event_tx.send(GameEvent::Tick).is_ok() {
                    last_tick = Instant::now();
                }
            }
        }
    });

    // Main loop
    loop {
        match rx.recv().unwrap() {
            GameEvent::Input(event) => {
                /* Which way is cleaner?
                A) Match by Scene / Game state -> Match by Event (has code duplication)
                B) Match by Event -> Filter by various Scenes/Game state (all code is together, kind of messy) 

                match simon.app_state.current_scene {
                    Scene::MainMenu => {}
                    Scene::Game => {
                        match event {
                            Event::Key(key) => {
                                match key.code {
                                    KeyCode::Esc => { 
                                        if simon.app_state.current_scene == Scene::MainMenu { return Ok(()) }
                                            simon.app_state.change_scene(Scene::MainMenu);
                                            simon.game_state.mode = GameMode::Preparing;
                                        }
                                    KeyCode::Up | KeyCode::Down | KeyCode::Left | KeyCode::Right => {
                                        simon.handle_keyboard_color_selection(key.code);
                                    }
                                    KeyCode::Char(c) =>  {
                                        if simon.game_state.mode != GameMode::GameOver {
                                            /* char commands */
                                            match c {
                                                'q' => { simon.app_state.is_paused = !simon.app_state.is_paused } // replace with Esc & a real pause menu
                                                'w' => { simon.handle_keyboard_color_selection(key.code); }
                                                'a' => { simon.handle_keyboard_color_selection(key.code); }
                                                's' => { simon.handle_keyboard_color_selection(key.code); }
                                                'd' => { simon.handle_keyboard_color_selection(key.code); }
                                                _ => {}
                                            }
                                        } else if simon.app_state.enable_text_entry {
                                            /* raw text input */
                                            if simon.score_state.new_score_name.len() >= simon.score_state.max_name_length.into() {
                                                continue; // name is too long!
                                            } else { simon.score_state.new_score_name.push(c); }
                                        }
                                    }
                                    KeyCode::Backspace => {
                                        if !simon.app_state.enable_text_entry { continue; } /* no action while not typing */ 
                                        else { simon.score_state.new_score_name.pop(); }
                                    }
                                    _ => {}
                                }
                            }
                            Event::Mouse(mouse) => {

                            }
                            _ => {}
                        }
                    }
                    Scene::Scores => {}
                    _ => {}
                }
                */
                match event {
                    Event::Key(key) => {
                        match key.code {
                            KeyCode::Esc => {
                                if simon.game_state.mode == GameMode::GameOver {
                                    // Escape from score input
                                    simon.game_state.mode = GameMode::Preparing; // should replace with GameState::new()
                                    simon.app_state.change_scene(Scene::MainMenu);
                                } else if simon.app_state.current_scene != Scene::MainMenu {
                                    simon.app_state.is_paused = !simon.app_state.is_paused;
                                }
                            }
                            // ToDo: Delete text_entry toggle - this should only toggle during score entry
                            KeyCode::Tab => { simon.app_state.enable_text_entry = !simon.app_state.enable_text_entry }
                            KeyCode::Backspace => {
                                if !simon.app_state.enable_text_entry { continue; } /* no action while not typing */
                                else { simon.score_state.new_score_name.pop(); }
                            }
                            KeyCode::Enter => {
                                if simon.game_state.mode == GameMode::GameOver {
                                    // save user name and exit
                                    let name = mem::take(&mut simon.score_state.new_score_name);
                                    let score = mem::take(&mut simon.game_state.current_score);
                                    
                                    if simon.score_state.is_name_new(&name) {
                                        simon.score_state.save_score(name, score);
                                        simon.debug_msg = String::new();
                                        simon.app_state.change_scene(Scene::MainMenu);
                                        simon.game_state.mode = GameMode::Preparing;
                                    } else {
                                        simon.debug_msg = String::from("That name is already taken");
                                    }
                                    
                                    // GameState::new() // reset the game
                                    continue;
                                }
                                select_menu_item(simon);
                            }
                            KeyCode::Up | KeyCode::Down | KeyCode::Left | KeyCode::Right
                                if simon.app_state.current_scene == Scene::Game && !simon.app_state.is_paused => {
                                    simon.handle_keyboard_color_selection(key.code);
                                }
                            KeyCode::Right | KeyCode::Down => { simon.select_next_list_item(); }
                            KeyCode::Left | KeyCode::Up => { simon.select_previous_list_item(); }
                            KeyCode::Char(c) =>  {
                                if !simon.app_state.enable_text_entry {
                                    /* char commands */
                                    if simon.app_state.current_scene != Scene::Game {
                                        if c.is_whitespace() { select_menu_item(simon); }
                                        match c {
                                            'w' | 'a' => { simon.select_previous_list_item(); }
                                            's' | 'd' => { simon.select_next_list_item(); }
                                            _ => {}
                                        }
                                    } else {
                                        if c.is_whitespace() { 
                                            if let Some(color) = simon.game_state.hovered_color {
                                                simon.handle_player_guess(color);
                                            }
                                        }
                                        match c {
                                            'w' | 'a' | 's' | 'd' => { 
                                                simon.handle_keyboard_color_selection(key.code); 
                                            }
                                            _ => {}
                                        }
                                    }
                                } else {
                                    /* raw text input */
                                    if simon.score_state.new_score_name.len() >= simon.score_state.max_name_length.into() {
                                        continue; // name is too long!
                                    } else { simon.score_state.new_score_name.push(c); }
                                }
                            }
                            _ => {}
                        }
                        
                    }
                    Event::Mouse(mouse) => {
                        simon.game_state.mouse_pos = (mouse.column, mouse.row);

                        if mouse.kind == event::MouseEventKind::Down(MouseButton::Left) {
                            if let Some(color) = simon.game_state.hovered_color {
                                simon.handle_player_guess(color);
                            }
                        }
                        
                        if mouse.kind == event::MouseEventKind::Moved && simon.game_state.mode == GameMode::AwaitingInput {
                            let pos = simon.game_state.mouse_pos.into();
                            let hovered_color = simon.game_state.clickables.iter().rev()
                                .find(|(_, r)| r.contains(pos))
                                .map(|(color, _)| *color);

                            if let Some(color) = hovered_color {                                
                                simon.game_state.handle_hovered_color(color);
                            } else {
                                // simon.debug_msg = format!("MISS! No color at {:?}", pos);
                            }
                        } else {
                            simon.game_state.hovered_color = None;
                        }
                    }
                    _ => {}
                }
            }
            GameEvent::Tick => {
                if simon.app_state.current_scene == Scene::Exit { return Ok(()) }

                simon.show_pattern(); 

                let _ = terminal.draw(|f| ui(f, simon));
            }
        }
    }
}

fn select_menu_item(simon: &mut Simon) {
    if let Some(selection) = simon.app_state.menu_list.state.selected() {
        let selected_scene = simon.app_state.menu_list.items[selection].scene;
        
        simon.app_state.change_scene(selected_scene);
        if simon.app_state.is_paused {
            simon.app_state.is_paused = false;
        }
    }
}