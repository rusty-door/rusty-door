use tickable::{Tickable,Input};
use labyrinth;
use std::time;
use std::fmt;

pub struct Game {
    pub field : labyrinth::Labyrinth,
    pub player : labyrinth::Point,
    pub playtime : time::Duration,
    pub walked : labyrinth::Field,
}

impl fmt::Debug for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut p = labyrinth::Field::new(self.walked.width(),
                                          self.walked.height(), false);
        p[self.player] = true;
        let s : String = format!("{:?}", self.field);
        let w : String = format!("{:?}", self.walked);
        let z : String = format!("{:?}", p);
        let r : String = s.chars().zip(w.chars()).zip(z.chars()).map(
                  |((x,y),z)| (x as u8 |
                               if y == '#' { '.' } else { y } as u8 |
                               if z == '#' { 'o' } else { z } as u8)
                               as char).collect();
        try!(write!(f, "{}\nTime: {:?}, player: {:?}\n", r,
              self.playtime, self.player));
        Ok(())
    }
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
        self.walked[self.player] = true;
        self.playtime += time::Duration::new(1, 0);
        match input {
            Some(Input::Direction(d)) => {
                let p = self.player.neighbor(d);
                if let Some(&false) = self.field.0.get(p) {
                    self.walked[self.player] = !self.walked[p];
                    self.player = p;
                }
            },
            _ => (),
        }
        None
    }

}

