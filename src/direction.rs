#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum LeftRight {
    Left = -1,
    Middle = 0,
    Right = 1,
}

impl LeftRight {
    fn from_int(i: i8) -> LeftRight {
        if i < 0 {
            LeftRight::Left
        } else if i > 0 {
            LeftRight::Right
        } else {
            LeftRight::Middle
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum UpDown {
    Up = -1,
    Middle = 0,
    Down = 1,
}

impl UpDown {
    fn from_int(i: i8) -> UpDown {
        if i < 0 {
            UpDown::Up
        } else if i > 0 {
            UpDown::Down
        } else {
            UpDown::Middle
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Direction(pub LeftRight, pub UpDown);

impl Direction {
    pub fn rot_cw(&self) -> Direction {
        match self.0 {
            LeftRight::Left => match self.1 {
                UpDown::Up     => Direction(LeftRight::Middle, UpDown::Up),
                UpDown::Middle => Direction(LeftRight::Left,   UpDown::Up),
                UpDown::Down   => Direction(LeftRight::Left,   UpDown::Middle)
            },
            LeftRight::Middle => match self.1 {
                UpDown::Up     => Direction(LeftRight::Right,  UpDown::Up),
                UpDown::Middle => Direction(LeftRight::Middle, UpDown::Middle),
                UpDown::Down   => Direction(LeftRight::Left,   UpDown::Down)
            },
            LeftRight::Right => match self.1 {
                UpDown::Up     => Direction(LeftRight::Right,  UpDown::Middle),
                UpDown::Middle => Direction(LeftRight::Right,  UpDown::Down),
                UpDown::Down   => Direction(LeftRight::Middle, UpDown::Down)
            }
        }
    }
}


