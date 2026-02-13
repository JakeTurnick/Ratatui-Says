use std::{
    io,
    error::Error,
    thread,
    time::Duration
};

use ratatui::{
    Terminal, backend::{
        Backend,
        CrosstermBackend
    }, crossterm::{
        event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyEventKind, KeyCode},
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
        Colors
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

    /* Todo - remove */ println!("{:?}", simon.current_pattern);

    let res = run_app(&mut terminal, &mut simon);

    // Clean up terminal
    // `?` pass errors back up to Box<dyn Error>
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), DisableMouseCapture, LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Ok(_exit_game) = res {
        println!("Thanks for playing!");
    } else if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, simon: &mut Simon) -> io::Result<bool> {
    loop {

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

        //terminal.draw(|f| ui(f, simon));


        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                println!("Mouse pressed!");
                continue;
            }

            match key.code {
                KeyCode::Esc => { return Ok(true); },
                _ => {}
            }
        } 
    }

}