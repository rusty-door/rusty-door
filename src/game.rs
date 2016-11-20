use tickable::{Tickable,Input};
use direction;
use labyrinth;
use std::time;

pub struct Game {
    pub field : labyrinth::Labyrinth,
    pub player : labyrinth::Point,
    pub playtime : time::Duration,
    pub walked : labyrinth::Field,
}

impl Game {
    pub fn new(width : usize, height : usize, seed: u16) -> Game {
        Game {
            field : labyrinth::Labyrinth::new(width, height, seed),
            player : labyrinth::Point { x : 0, y : 0 },
            playtime : time::Duration::new(0, 0),
            walked : labyrinth::Field::new(width, height, false),
        }
    }
}

impl Tickable for Game {

    fn tick(&mut self, input: Option<Input>) -> Option<Box<Tickable>> {
        None
    }
}

