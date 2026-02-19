use std::{
    error::Error, io, thread, time::{Duration, Instant}
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
        Bounds_2d
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
    simon.add_to_pattern(4); // Start with 1 color
    simon.game_state.showing_pattern = true;

    let last_tick = Instant::now();
    let tick_rate = Duration::from_millis(16);
    
    let res = run_app(&mut terminal, &mut simon, last_tick, tick_rate);

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

fn run_app<B: Backend>(terminal: &mut Terminal<B>, simon: &mut Simon, mut last_tick: Instant, tick_rate: Duration) -> io::Result<bool> {
    loop {

        let start_time = Instant::now();
        // THIS WILL ADD ON EVERY INPUT - 
        // todo: make some state for "actively_inputting" or something
        //simon.add_to_pattern(3);

        /* - WORKS - colors appear on screen, commented out to work on click event
        if simon.game_state.showing_pattern == true {
            for (i, color) in simon.current_pattern.iter().enumerate() {
                simon.game_state.shown_color = Some(*color); // they say it's wrong but idk how to be right
                
                terminal.draw(|f| ui(f, simon));

                // draw blank with pause, duplicate colors show each as a flash
                thread::sleep(Duration::from_secs_f32(0.5));
                simon.game_state.shown_color = None;
                terminal.draw(|f| ui(f, simon));
                thread::sleep(Duration::from_secs_f32(0.5));
            }
            simon.game_state.shown_color = None;
            simon.game_state.showing_pattern = false;
        } 
        else if simon.game_state.awaiting_input == true {
            // Take mouse / key input for colors

            simon.game_state.awaiting_input = false;
        }*/


        

        while event::poll(Duration::from_millis(0))? {
            /*
                Try updating a mouse_selection property on mouse

                How would this be different from getting mouse_pos on click event,
                without making a button type to select?

                By updating color selection
             */
            if let Event::Mouse(mouse) = event::read()? {
                if mouse.kind == event::MouseEventKind::Moved {
                    simon.game_state.mouse_pos = (mouse.column, mouse.row);

                    
                }
                if mouse.kind == event::MouseEventKind::Down(MouseButton::Left) {
                    //simon.debug_msg= format!("left click at {:?}, {:?}", mouse.column, mouse.row);
                    if let Some(color) = simon.game_state.selected_color {
                        simon.debug_msg = format!("click detected! color: {}", color);
                    }
                    

                    /*
                    let mouse_pos = (mouse.column, mouse.row);

                    if let Some((color, _rect)) = simon.game_state.clickables.iter().rev().find(|(_, r)| r.contains(mouse_pos.into())) {
                        match color {
                            Colors::RED => {
                                simon.debug_msg = String::from("clicked red!");
                                simon.game_state.missed_clicks = 0;
                            },
                            Colors::YELLOW => {
                                simon.debug_msg = String::from("clicked yellow!");
                                simon.game_state.missed_clicks = 0;
                            },
                            Colors::GREEN => {
                                simon.debug_msg = String::from("clicked green!");
                                simon.game_state.missed_clicks = 0;
                            },
                            Colors::BLUE => {
                                simon.debug_msg = String::from("clicked blue!");
                                simon.game_state.missed_clicks = 0;
                            },
                            _ => { }
                        }
                    }
                    simon.game_state.missed_clicks += 1;
                    */
                }
            }

            if let Event::Key(key) = event::read()? {
                if key.kind == event::KeyEventKind::Release {
                    continue;
                }

                match key.code {
                    KeyCode::Esc => { return Ok(true); },
                    KeyCode::Char('q') => { println!("\rIts a Q"); }
                    KeyCode::Char('e') => { println!("");}
                    _ => {}
                }
            }

        }

        if last_tick.elapsed() >= tick_rate {
            if let Some((color, _rect)) = simon.game_state.clickables.iter().rev().find(|(_, r)| r.contains(simon.game_state.mouse_pos.into())) {
                simon.game_state.selected_color = Some(*color);
                simon.debug_msg = format!("Hovered color: {:?} at {:?}", simon.game_state.selected_color, simon.game_state.mouse_pos);
            }

            terminal.draw(|f| ui(f, simon))?;
            last_tick = Instant::now();

            simon.game_state.last_frame_time = start_time.elapsed();
        }
        std::thread::sleep(Duration::from_millis(1));
        
    }

}