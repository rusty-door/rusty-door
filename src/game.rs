use tickable::{Tickable,Input};
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
            player : labyrinth::Point { x : 1, y : 1 },
            playtime : time::Duration::new(0, 0),
            walked : labyrinth::Field::new(width, height, false),
        }
    }

    pub fn is_finished(&self) -> bool {
        self.player == (labyrinth::Point {
            x : self.field.0.height() - 1,
            y : self.field.0.width()  - 1
        })
    }
}

impl Tickable for Game {

    fn tick(&mut self, input: Option<Input>) -> Option<Box<Tickable>> {
        self.playtime += time::Duration::new(1, 0);
        match input {
            Some(Input::Direction(d)) => {
                let p = self.player.neighbor(d);
                if let Some(&false) = self.field.0.get(p) {
                    self.player = p;
                }
            },
            _ => (),
        }
        None
    }

}

