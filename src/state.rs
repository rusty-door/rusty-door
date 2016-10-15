use std::time;

use labyrinth;
use screen;
use menu;

pub struct GameState {
    field : labyrinth::Labyrinth,
    player : labyrinth::Point,
    playtime : time::Duration,
}

impl GameState {
    pub fn new(width : usize, height : usize) -> GameState {
        GameState {
            field : labyrinth::Labyrinth::new(width, height),
            player : labyrinth::Point { x : 0, y : 0 },
            playtime : time::Duration::new(0, 0)
        }
    }
}

pub struct ScoreEntry {
    player : String,
    score : u32
}

pub struct Score {
    entries : Vec<ScoreEntry>
}

impl Score {
    fn new() -> Score {
        Score {
            entries : Vec::new()
        }
    }
}

pub struct ProgramState {
    game : Option<GameState>,
    score : Score,
    screen : Box<screen::Screen>,
    width : usize,
    height : usize
}

impl ProgramState {
    pub fn new() -> ProgramState {
        ProgramState {
            game : None,
            screen : Box::new(menu::MenuScreen::new()),
            score : Score::new(),
            width : 50,
            height : 30
        }
    }
}

