use std::time;

use labyrinth;

pub struct GameState {
    pub field : labyrinth::Labyrinth,
    pub player : labyrinth::Point,
    pub playtime : time::Duration,
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
    pub player : String,
    pub score : u32
}

pub struct Score {
    pub entries : Vec<ScoreEntry>
}

impl Score {
    fn new() -> Score {
        Score {
            entries : Vec::new()
        }
    }
}

pub struct ProgramState {
    pub game : Option<GameState>,
    pub score : Score,
    pub width : usize,
    pub height : usize
}

impl ProgramState {
    pub fn new() -> ProgramState {
        ProgramState {
            game : None,
            score : Score::new(),
            width : 70,
            height : 30
        }
    }

    pub fn new_game(&mut self) {
        self.game = Some(GameState::new(self.width, self.height))
    }
}

