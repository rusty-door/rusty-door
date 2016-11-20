use direction;

#[derive (Clone, Copy)]
pub enum Input {
    Accept,
    Cancel,
    Direction(direction::Direction),
    Menu,
}

