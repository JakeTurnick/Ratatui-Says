use rand;
use std::{
    fmt, fs::{self, create_dir_all}, path::PathBuf, time::{Duration, Instant}
};
use ratatui::{
    crossterm::event::{self, KeyCode}, layout::Rect, widgets::ListState
};
use serde::{Serialize, Deserialize};


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
    fn from_index(index: u8) -> Option<Colors> {
        match index {
            0 => Some(Colors::RED),
            1 => Some(Colors::YELLOW),
            2 => Some(Colors::GREEN),
            3 => Some(Colors::BLUE),
            _ => None,
        }
    }

    fn to_index(c: Colors) -> u8 {
        match c {
            Colors::RED => 0,
            Colors::YELLOW => 1,
            Colors::GREEN => 2,
            Colors::BLUE => 3,
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
        let mut state = ListState::default();
        state.select_first();
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
                ("Exit game", Scene::Exit),
            ]),
            is_paused: false,
            enable_text_entry: false
        }
    }

    pub fn change_scene(&mut self, scene: Scene) {
        match scene {
            Scene::MainMenu => { self.current_scene = Scene::MainMenu }
            Scene::Game => { self.current_scene = Scene::Game }
            Scene::Scores => { self.current_scene = Scene::Scores }
            Scene::Exit => { self.current_scene = Scene::Exit }
        }

        self.menu_list = MenuList::from_iter([
                ("Main Menu", Scene::MainMenu),
                ("Play", Scene::Game),
                ("Scores", Scene::Scores),
                ("Exit game", Scene::Exit),
            ]);
        self.menu_list.items.retain(|item| {
            item.scene != scene
        });
    }
}

pub struct GameState {
    pub mode: GameMode,
    pub current_pattern: Vec<Colors>,
    pub shown_color: Option<Colors>,
    pub hovered_color: Option<Colors>,
    pub mouse_pos: (u16, u16),
    pub clickables: Vec<(Colors, Rect)>,
    pub current_score: u8
}

impl GameState {
    pub fn new() -> GameState {
        GameState { 
            mode: GameMode::Preparing,
            current_pattern: Vec::new(),
            shown_color: None,
            hovered_color: None,
            mouse_pos: (0, 0),
            clickables: vec!(),
            current_score: 0
        }
    }

    pub fn handle_hovered_color(&mut self, color: Colors) {
        if self.mode != GameMode::AwaitingInput {
            return;
        }
        self.hovered_color = Some(color);
    }

    pub fn add_to_pattern(pattern: &mut Vec<Colors>, iterations: i8) {
        for _i in 0..iterations {
            let new_color = Colors::from_index(rand::random_range(0..=3)).expect("Random range should be within bounds of hard-coded enum");
            pattern.push(new_color);
        }
    }

    pub fn handle_keyboard_color_selection(&mut self, direction: KeyCode) {
        if self.mode != GameMode::AwaitingInput { return }

        const WIDTH: u8 = 2;

        let mut row: u8 = 0;
        let mut col: u8 = 0;
        let mut index: u8;

        if let Some(color) = self.hovered_color {
            index = Colors::to_index(color);
            row = index / WIDTH;
            col = index % WIDTH;
        }
        
        match direction {
            KeyCode::Up | KeyCode::Char('w') => { row = row.saturating_sub(1); }
            KeyCode::Down | KeyCode::Char('s') => { row = row.saturating_add(1); }
            KeyCode::Left | KeyCode::Char('a') => { col = col.saturating_sub(1); }
            KeyCode::Right | KeyCode::Char('d') => { col = col.saturating_add(1); }
            _ => {}
        }

        row = row.clamp(0, 1);
        col = col.clamp(0, 1);

        index = (row * WIDTH) + col;

        //self.debug_msg = format!("color index: {:?}", index);
        self.hovered_color = Colors::from_index(index);
    }
}

#[derive(PartialEq)]
pub enum GameMode {
    Preparing,
    ShowingPattern,
    AwaitingInput,
    GameOver,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ScoreItem {
    pub score: u8,
    pub name: String
}

impl ScoreItem {
    fn new(name: String, score: u8) -> Self {
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

impl FromIterator<(String, u8)> for ScoreList {
    fn from_iter<T: IntoIterator<Item = (String, u8)>>(iter: T) -> Self {
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
}

impl ScoreState {
    fn new() -> Self {
        let path = Self::get_app_path();
        let mut initial_scores = Vec::new();

        let raw_read = fs::read_to_string(&path);

        match raw_read {
            Ok(data) => {
                if let Ok(decoded) = serde_json::from_str::<Vec<ScoreItem>>(&data) {
                    initial_scores = decoded;
                }
            },
            Err(err) => {
                eprintln!("Error: {:?}", err);

            }
        }

        let mut instance = Self {
            score_list: ScoreList {
                scores: initial_scores,
                state: ListState::default(),
            },
            new_score_name: String::new(),
            max_name_length: 8,
        };

        instance.sort_scores();
        instance
    }

    fn sort_scores(&mut self) {
        // Sort descending: highest scores first
        self.score_list.scores.sort_by(|a, b| b.score.cmp(&a.score));
    }

    fn get_app_path() -> PathBuf {
        use directories::ProjectDirs;
        let mut path = ProjectDirs::from("com", "yourname", "simon_game")
            .map(|proj| proj.data_dir().join("data"))
            .expect("Could not determine save directory");

        create_dir_all(&path).expect("Could not create save directory");

        let _ = path.push("scores.json");

        //let _file = fs::File::create(&path).expect("Could not create file or open scores.json");

        path
    }

    pub fn save_score(&mut self, name: String, score: u8) {
        let new_score = ScoreItem::new(name, score);

        self.score_list.scores.push(new_score);
        self.sort_scores();

        let path = Self::get_app_path();
        let json_content = serde_json::to_string_pretty(&self.score_list.scores).expect("Scores should convert to string from Vec<String, u8>");

        let _write_result = fs::write(&path, json_content);
    }

    pub fn is_name_new(&self, new_name: &String) -> bool {
        for score in &self.score_list.scores {
            if &score.name == new_name {
                return false
            }
        }
        return true
    }
}


pub struct Simon {
    pub step_index: usize,          // index of current_pattern
    pub last_step_time: Instant,    // color flash timings
    pub game_state: GameState,
    pub app_state: AppState,
    pub score_state: ScoreState,
    pub debug_msg: String,
}

impl Simon {
    pub fn new() -> Simon {
        Simon {
            //current_pattern: Vec::new(),
            step_index: 0,
            last_step_time: Instant::now(),
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
        if self.app_state.is_paused {
            return
        }

        match self.game_state.mode {
            GameMode::Preparing => {
                // wait 1 second, start the sequence
                if self.last_step_time.elapsed() > Duration::from_millis(200) {
                    self.game_state.shown_color = None;
                }
                if self.last_step_time.elapsed() > Duration::from_millis(1000) {
                    self.game_state.mode = GameMode::ShowingPattern;
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
                    if self.step_index >= self.game_state.current_pattern.len() {
                        self.game_state.mode = GameMode::AwaitingInput;
                        self.game_state.shown_color = None;
                        self.game_state.hovered_color = Some(Colors::RED);
                        self.step_index = 0;
                    }
                } else if elapsed > Duration::from_millis(700) {
                    self.game_state.shown_color = None; // The "gap" between flashes
                } else {
                    if let Some(&color) = self.game_state.current_pattern.get(self.step_index) {
                        self.game_state.shown_color = Some(color);
                    }
                }
            },
            _ => return
        }   
    }

    // Not in game_state because GameOver logic changes app behaviro (text entry)
    pub fn handle_player_guess(&mut self, color: Colors) {
        if self.game_state.mode != GameMode::AwaitingInput {
            return;
        }
        self.game_state.shown_color = Some(color);
        self.last_step_time = Instant::now();
        if color == self.game_state.current_pattern[self.step_index] {
            self.game_state.current_score += 1;
            self.debug_msg = format!("Correct! Score: {}", self.game_state.current_score);
            
            self.step_index += 1;

            if self.step_index >= self.game_state.current_pattern.len() {
                self.debug_msg = format!("New pattern!");
                //self.add_to_pattern(1);
                GameState::add_to_pattern(&mut self.game_state.current_pattern, 1);

                self.game_state.mode = GameMode::Preparing;
            } 
        } else {
            self.debug_msg = String::new();
            self.game_state.mode = GameMode::GameOver;
            self.app_state.enable_text_entry = true;
        }
    }

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