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
    Down = -1,
    Middle = 0,
    Up = 1,
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

    fn rot_by(&self, re: i8, im: i8) -> Direction {
        let Direction(x, y) = *self;
        let (a, b) = (x as i8, y as i8);
        Direction(
            LeftRight::from_int(a * re - b * im),
            UpDown::from_int(a * im - b * re))
    }

    pub fn rot_cw(&self) -> Direction {
        self.rot_by(1, -1)
    }

    pub fn rot_ctr_cw(&self) -> Direction {
        self.rot_by(1, 1)
    }
}

