use rand;
use std::fmt;

#[derive(Debug)]
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

pub struct Simon {
    level: i8,
    pub current_pattern: Vec<Colors>,

}

impl Simon {
    pub fn new() -> Simon {
        let starting_pattern = vec!(Colors::from_index(rand::random_range(0..=3)).expect("Random range should be within bounds of hard-coded enum"));
        Simon {
            level: 1,
            current_pattern: starting_pattern
        }
    }

    // really just for debugging and testing Display and from_index()
    pub fn new_pattern(&self)  {
        let starting_color = rand::random_range(0..=3);
        let color = Colors::from_index(starting_color).unwrap();
        println!("{starting_color}, is {color}");
    }

    pub fn add_to_pattern(&mut self, iterations: i8) {
        for _i in 1..iterations {
            let new_color = Colors::from_index(rand::random_range(0..=3)).expect("Random range should be within bounds of hard-coded enum");
            self.current_pattern.push(new_color);
        }
    }

}