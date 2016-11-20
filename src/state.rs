use game::Game;

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
    pub game : Option<Game>,
    pub score : Score,
    pub width : usize,
    pub height : usize,
    pub seed : Option<u16>
}

impl ProgramState {
    pub fn new() -> ProgramState {
        ProgramState {
            game : None,
            score : Score::new(),
            width : 70,
            height : 30,
            seed : None,
        }
    }

    pub fn new_game(&mut self) {
        self.game = Some(Game::new(self.width, self.height,
                                        self.seed.unwrap_or(0)))
    }
}

