use rand;
use std::fmt;

#[derive(Debug, Clone, Copy)]
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

pub struct Game_State {
    pub showing_pattern: bool,
    pub awaiting_input: bool,
    pub shown_color: Option<Colors>
}

impl Game_State {
    pub fn new() -> Game_State {
        Game_State { 
            showing_pattern: false, 
            awaiting_input: false,
            shown_color: None
        }
    }
}

pub struct Simon {
    level: i8,
    pub current_pattern: Vec<Colors>,
    pub game_state: Game_State
}

impl Simon {
    pub fn new() -> Simon {
        let starting_pattern = vec!(Colors::from_index(rand::random_range(0..=3)).expect("Random range should be within bounds of hard-coded enum"));
        Simon {
            level: 1,
            current_pattern: starting_pattern,
            game_state: Game_State::new()
        }
    }

    pub fn add_to_pattern(&mut self, iterations: i8) {
        for _i in 1..iterations {
            let new_color = Colors::from_index(rand::random_range(0..=3)).expect("Random range should be within bounds of hard-coded enum");
            self.current_pattern.push(new_color);
        }
    }

}