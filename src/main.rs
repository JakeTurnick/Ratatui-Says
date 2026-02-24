use std::{
    error::Error, io, time::{Duration, Instant},
    sync::mpsc,
    thread
};

use ratatui::{
    Terminal, backend::{
        Backend,
        CrosstermBackend
    }, crossterm::{
        event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyEventKind, KeyCode, MouseButton},
        execute,
        terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode}
    } 
    //prelude::CrosstermBackend
};

mod app;
mod ui;
use crate::{
    app::{
        Simon,
        Colors,
        Game_Event,
        Game_Mode
    },
    ui::ui
};


fn main() -> Result<(), Box<dyn Error>> {
    // Set up terminal
    enable_raw_mode()?;
    let mut stderr = io::stderr();
    execute!(stderr, EnableMouseCapture, EnterAlternateScreen);

    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    // Create & Start simon App
    let mut simon = Simon::new();
    simon.add_to_pattern(4); // Start with 4 colors
    
    let res = run_app(&mut terminal, &mut simon);

    // Clean up terminal
    // `?` pass errors back up to Box<dyn Error>
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), DisableMouseCapture, LeaveAlternateScreen)?;
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
                    event_tx.send(Game_Event::Input(ev)).unwrap();
                }
            }

            // reset current tick
            if last_tick.elapsed() >= tick_rate {
                if event_tx.send(Game_Event::Tick).is_ok() {
                    last_tick = Instant::now();
                }
            }
        }
    });

    // Main loop
    loop {
        match rx.recv().unwrap() {
            Game_Event::Input(event) => {
                match event {
                    Event::Key(key) => {
                        // Exit game
                        if key.code == KeyCode::Esc { return Ok(()); }
                    }
                    Event::Mouse(mouse) => {
                        simon.game_state.mouse_pos = (mouse.column, mouse.row);

                        if mouse.kind == event::MouseEventKind::Down(MouseButton::Left) {
                            let pos = simon.game_state.mouse_pos.into();
                            let clicked_color = simon.game_state.clickables.iter().rev()
                                .find(|(_, r)| r.contains(pos))
                                .map(|(color, _)| *color);

                            if let Some(color) = clicked_color {                                
                                simon.handle_player_guess(color);
                            } else {
                                simon.debug_msg = format!("MISS! No color at {:?}", pos);
                            }
                        }
                    }
                    _ => {}
                }
            }
            Game_Event::Tick => {
                simon.show_pattern(); 

                terminal.draw(|f| ui(f, simon))?;
            }
        }
    }
}