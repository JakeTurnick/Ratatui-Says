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
    current_pattern: Vec<Colors>,

}

impl Simon {
    pub fn new() -> Simon {
        Simon {
            level: 1,
            current_pattern: Vec::new()
        }
    }

    pub fn new_pattern(&self)  {
        let starting_color = rand::random_range(0..=3);
        let color = Colors::from_index(starting_color).unwrap();
        println!("{starting_color}, is {color}");
    }

    fn add_to_pattern(&mut self) -> Vec<Colors> {
        // self.current_pattern = ...
        todo!()
    }

}