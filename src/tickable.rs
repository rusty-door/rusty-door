use direction::Direction;

#[derive (Clone, Copy)]
pub enum Input {
    Accept,
    Cancel,
    Direction(Direction),
    Menu,
}

pub trait Tickable {
    fn tick(&mut self, Option<Input>) -> Option<Box<Tickable>>;
}

