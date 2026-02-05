use std::{
    io,
    error::Error
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
    let res = run_app(&mut terminal, simon);

    // Clean up terminal
    // `?` pass errors back up to Box<dyn Error>
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), DisableMouseCapture, LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Ok(exit_game) = res {
        println!("Thanks for playing!");
    } else if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, simon: Simon) -> io::Result<bool> {
    loop {
        //terminal.draw(|f| ui(f, &simon))?;

        simon.new_pattern();
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                continue;
            }

            match key.code {
                KeyCode::Char('q') => { return Ok(true); }
                _ => {}
            }
        } 
    }

    todo!()
}