mod labyrinth;
mod menu;
mod tickable;
mod state;
mod direction;
mod game;
mod play;
mod license;
mod screen;
mod geometry;
mod render;

use tickable::Input;
use screen::Screen;
use direction::{DIR_UP,DIR_DOWN,DIR_LEFT,DIR_RIGHT};
use std::io::{Read,stdin};

fn main() {
    let pr = state::ProgramState::new();
    let mut scr : Box<Screen> = Box::new(menu::MenuScreen::new(pr));
    for b in stdin().bytes() {
        let c = b.unwrap_or(0) as char;
        if c != '\n' && c != '\0' {
            let s = match c {
                'h' => Some(Input::Direction(DIR_LEFT)),
                'j' => Some(Input::Direction(DIR_DOWN)),
                'k' => Some(Input::Direction(DIR_UP)),
                'l' => Some(Input::Direction(DIR_RIGHT)),
                'z' => Some(Input::Accept),
                'x' => Some(Input::Cancel),
                ':' => Some(Input::Menu),
                _   => None
            };
            if let Some(f) = scr.tick(s) {
                scr = f;
            }
            println!("{:?}", scr);
        }
    }
}

