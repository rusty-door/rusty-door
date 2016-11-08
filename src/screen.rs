use direction;
use state;

pub enum UserInput {
    Accept,
    Cancel,
    Direction(direction::Direction),
    Menu,
}

pub trait Screen {
    fn tick(&mut self, Option<UserInput>) -> Option<Box<Screen>>;
}

