mod labyrinth;
mod menu;
mod tickable;
mod state;
mod direction;
mod game;
mod play;
mod license;
mod screen;

use tickable::Input;
use screen::Screen;

fn main() {
    let pr = state::ProgramState::new();
    let mut scr : Box<Screen> = Box::new(menu::MenuScreen::new(pr));
    for input in [Input::Accept,
                  Input::Cancel,
                  Input::Direction(direction::Direction(
                      direction::LeftRight::Left, direction::UpDown::Up)),
                  Input::Menu].iter() {
            if let Some(f) = scr.tick(Some(*input)) {
                scr = f;
            }
            println!("{:?}", scr);
    }
}

