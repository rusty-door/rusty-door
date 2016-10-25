use direction;
use state;

pub enum UserInput {
    Accept,
    Cancel,
    Direction(direction::Direction),
    Menu,
}

pub trait Screen {
    fn tick<'a>(&'a mut self, Option<UserInput>) -> &'a mut Screen;
    fn new<'a>(state: &'a mut state::ProgramState) -> Box<Screen + 'a>
        where Self: Sized;
}

