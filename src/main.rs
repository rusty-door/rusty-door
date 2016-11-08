mod labyrinth;
mod menu;
mod screen;
mod state;
mod direction;

use screen::Screen;
use screen::UserInput;

fn main() {
    let mut pr = state::ProgramState::new();
    let mut scr : Box<Screen> = Box::new(menu::MenuScreen::new(&mut pr));
    for input in [UserInput::Accept,
                  UserInput::Cancel,
                  UserInput::Direction(direction::Direction(
                      direction::LeftRight::Left, direction::UpDown::Up)),
                  UserInput::Menu].iter() {
            if let Some(f) = scr.tick(Some(*input)) {
                scr = f;
            }
    }
}

