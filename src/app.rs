use rand;
use std::{
    fmt, 
    time::{Duration, Instant},
};
use ratatui::{
    layout::Rect,
    crossterm::event,
    widgets::{ListState}
};
use serde::{Serialize, Deserialize};
use std::path::PathBuf;
use std::fs;

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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Scene {
    MainMenu,
    Game,
    Scores,
    Exit
}

#[derive(Debug)]
pub struct MenuItem {
    pub name: &'static str,
    pub scene: Scene
}

impl MenuItem {
    fn new(name: &'static str, scene: Scene) -> MenuItem {
        MenuItem {
            name,
            scene
        }
    }
}

pub struct MenuList {
    pub items: Vec<MenuItem>,
    pub state: ListState
}

impl FromIterator<(&'static str, Scene)> for MenuList {
    fn from_iter<T: IntoIterator<Item = (&'static str, Scene)>>(iter: T) -> Self {
        let items = iter
            .into_iter()
            .map(|(name, scene)| MenuItem::new(name, scene))
            .collect();
        let state = ListState::default();
        Self { items, state }
    }
}

pub struct AppState {
    pub current_scene: Scene,
    pub menu_list: MenuList,
    pub is_paused: bool,
    pub enable_text_entry: bool
}

impl AppState {
    fn new() -> AppState {
        AppState {
            current_scene: Scene::MainMenu,
            menu_list: MenuList::from_iter([
                ("Play", Scene::Game),
                ("Scores", Scene::Scores),
                ("Exit game", Scene::Exit)
            ]),
            is_paused: false,
            enable_text_entry: false
        }
    }

    pub fn change_scene(&mut self, scene: Scene) {
        match scene {
            Scene::Game => { self.current_scene = Scene::Game }
            Scene::MainMenu => { self.current_scene = Scene::MainMenu }
            Scene::Scores => { self.current_scene = Scene::Scores }
            Scene::Exit => { self.current_scene = Scene::Exit }
        }
    }
}

pub struct GameState {
    pub shown_color: Option<Colors>,
    pub mouse_pos: (u16, u16),
    pub clickables: Vec<(Colors, Rect)>,
    pub current_score: u16
}

impl GameState {
    pub fn new() -> GameState {
        GameState { 
            shown_color: None,
            mouse_pos: (0, 0),
            clickables: vec!(),
            current_score: 0
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

// Should hold overarchin state and info
// TODO - move game state & logic into game_state
// ^-- GOAL: start new game with `simon.game_state = GameState::new()`


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ScoreItem {
    pub score: usize,
    pub name: String
}

impl ScoreItem {
    fn new(name: String, score: usize) -> Self {
        ScoreItem {
            name,
            score
        }
    }
}

pub struct ScoreList {
    pub scores: Vec<ScoreItem>,
    pub state: ListState
}

impl FromIterator<(String, usize)> for ScoreList {
    fn from_iter<T: IntoIterator<Item = (String, usize)>>(iter: T) -> Self {
        let scores = iter
            .into_iter()
            .map(|(name, score)| ScoreItem::new(name, score))
            .collect();
        let state = ListState::default();
        Self { scores, state }
    }
}

pub struct ScoreState {
    pub score_list: ScoreList,
    pub new_score_name: String,
    pub max_name_length: u8,
    path: PathBuf
}

impl ScoreState {
    fn new() -> Self {
        let path = Self::get_app_path(); // Logic is now internal
        let mut initial_scores = Vec::new();

        if let Ok(data) = fs::read_to_string(&path) {
            if let Ok(decoded) = serde_json::from_str::<Vec<ScoreItem>>(&data) {
                initial_scores = decoded;
            }
        }

        let dummy_list = ScoreList::from_iter([
            (String::from("AAA"), 1),
            (String::from("BBB"), 2),
            (String::from("CCC"), 3),
            (String::from("DDD"), 4),
        ]);

        let mut instance = Self {
            score_list: dummy_list,
            new_score_name: String::new(),
            max_name_length: 8,
            path,
        };

        /* let mut instance = Self {
            score_list: ScoreList {
                scores: initial_scores,
                state: ListState::default(),
            },
            path,
        }; */
        
        instance.sort_scores();
        instance
    }

    fn sort_scores(&mut self) {
        // Sort descending: highest scores first
        self.score_list.scores.sort_by(|a, b| b.score.cmp(&a.score));
    }

    fn get_app_path() -> PathBuf {
        use directories::ProjectDirs;
        ProjectDirs::from("com", "yourname", "simon_game")
            .map(|proj| proj.data_dir().join("scores.json"))
            .expect("Could not determine save directory")
    }
}


pub struct Simon {
    pub current_pattern: Vec<Colors>,
    pub step_index: usize,          // index of current_pattern
    pub last_step_time: Instant,    // color flash timings
    pub mode: GameMode,
    pub game_state: GameState,
    pub app_state: AppState,
    pub score_state: ScoreState,
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
            app_state: AppState::new(),
            score_state: ScoreState::new(),
            debug_msg: String::from("debug")
        }
    }

    // Simon's turn
    // plays pattern in step increments
    // returns to Awaiting_input
    pub fn show_pattern(&mut self) {

        if self.app_state.current_scene != Scene::Game { 
            return 
        }

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

    pub fn add_to_pattern(pattern: &mut Vec<Colors>, iterations: i8) {
        for _i in 0..iterations {
            let new_color = Colors::from_index(rand::random_range(0..=3)).expect("Random range should be within bounds of hard-coded enum");
            pattern.push(new_color);
        }
    }

    pub fn handle_player_guess(&mut self, color: Colors) {
        if self.mode != GameMode::AwaitingInput {
            return;
        }
        if color == self.current_pattern[self.step_index] {
            self.game_state.current_score += 1;
            self.debug_msg = format!("Correct! Score: {}", self.game_state.current_score);
            
            self.step_index += 1;

            if self.step_index >= self.current_pattern.len() {
                self.debug_msg = format!("New pattern!");
                //self.add_to_pattern(1);
                Simon::add_to_pattern(&mut self.current_pattern, 1);

                self.mode = GameMode::Preparing;
            } 
        } else {
            self.debug_msg = format!("Wrong Guess - Game over!");
            self.mode = GameMode::GameOver;
        }
    }

    // List functions

    pub fn select_next_list_item(&mut self) {
        if self.app_state.menu_list.state.selected().unwrap_or(0) >= self.app_state.menu_list.items.len() - 1 {
            self.app_state.menu_list.state.select_first();
        } else {
            self.app_state.menu_list.state.select_next();
        }
        
    }

    pub fn select_previous_list_item(&mut self) {
        if self.app_state.menu_list.state.selected() == 0.into() {
            self.app_state.menu_list.state.select_last();
        } else {
            self.app_state.menu_list.state.select_previous();
        }
        
    }

}