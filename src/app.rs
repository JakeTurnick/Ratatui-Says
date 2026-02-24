use rand;
use std::{
    fmt, 
    time::{Duration, Instant},
    thread
};
use ratatui::{
    layout::Rect,
    crossterm::event
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Colors {
    RED,
    YELLOW,
    BLUE,
    GREEN
}

impl fmt::Display for Colors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Colors::RED => write!(f, "Red"),
            Colors::YELLOW => write!(f, "Yellow"),
            Colors::GREEN => write!(f, "Green"),
            Colors::BLUE => write!(f, "Blue")
        }
    }
}

impl Colors {
    fn from_index(index: usize) -> Option<Colors> {
        match index {
            0 => Some(Colors::RED),
            1 => Some(Colors::YELLOW),
            2 => Some(Colors::GREEN),
            3 => Some(Colors::BLUE),
            _ => None,
        }
    }
}

pub enum GameEvent {
    Input(event::Event),
    Tick
}

pub struct GameState {
    pub shown_color: Option<Colors>,
    pub mouse_pos: (u16, u16),
    pub clickables: Vec<(Colors, Rect)>,
}

impl GameState {
    pub fn new() -> GameState {
        GameState { 
            shown_color: None,
            mouse_pos: (0, 0),
            clickables: vec!(),
        }
    }
}

#[derive(PartialEq)]
pub enum GameMode {
    Preparing,
    ShowingPattern,
    AwaitingInput,
    GameOver,
}

pub struct Simon {
    pub current_pattern: Vec<Colors>,
    pub step_index: usize,          // index of current_pattern
    pub last_step_time: Instant,    // color flash timings
    pub mode: GameMode,
    pub game_state: GameState,
    pub debug_msg: String,
}

impl Simon {
    pub fn new() -> Simon {
        Simon {
            current_pattern: Vec::new(),
            step_index: 0,
            last_step_time: Instant::now(),
            mode: GameMode::Preparing,
            game_state: GameState::new(),
            debug_msg: String::from("debug")
        }
    }

    // Simon's turn
    // plays pattern in step increments
    // returns to Awaiting_input
    pub fn show_pattern(&mut self) {

        match self.mode {
            GameMode::Preparing => {
                // wait 1 second, start the sequence
                if self.last_step_time.elapsed() > Duration::from_millis(1000) {
                    self.mode = GameMode::ShowingPattern;
                    self.step_index = 0;
                    self.last_step_time = Instant::now();
                }
            }, 
            GameMode::ShowingPattern => {
                //self.debug_msg = format!("showing parttern: (Step, Len) ({}, {})", self.step_index, self.current_pattern.len() - 1);
                let elapsed = self.last_step_time.elapsed();
                
                // 1000ms total cycle: 700ms ON, 300ms OFF
                if elapsed > Duration::from_millis(1000) {
                    self.step_index += 1;
                    self.last_step_time = Instant::now();

                    // Reset to Player's turn
                    if self.step_index >= self.current_pattern.len() {
                        self.mode = GameMode::AwaitingInput;
                        self.game_state.shown_color = None;
                        self.step_index = 0;
                    }
                } else if elapsed > Duration::from_millis(700) {
                    self.game_state.shown_color = None; // The "gap" between flashes
                } else {
                    if let Some(&color) = self.current_pattern.get(self.step_index) {
                        self.game_state.shown_color = Some(color);
                    }
                }
            },
            _ => return
        }   
    }

    pub fn add_to_pattern(&mut self, iterations: i8) {
        for _i in 0..iterations {
            let new_color = Colors::from_index(rand::random_range(0..=3)).expect("Random range should be within bounds of hard-coded enum");
            self.current_pattern.push(new_color);
        }
    }

    pub fn handle_player_guess(&mut self, color: Colors) {
        if self.mode != GameMode::AwaitingInput {
            return;
        }
        if color == self.current_pattern[self.step_index] {
            self.debug_msg = format!("Correct guess! {} - (Step, Len) ({}, {})", color, self.step_index, self.current_pattern.len());
            self.step_index += 1;

            if self.step_index >= self.current_pattern.len() {
                self.debug_msg = format!("New pattern!");
                self.add_to_pattern(1);

                self.mode = GameMode::Preparing;
            } 
        } else {
            self.debug_msg = format!("Wrong Guess - Game over!");
            self.mode = GameMode::GameOver;
        }
    }

}