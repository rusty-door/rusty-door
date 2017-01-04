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
        if i > 0 {
            UpDown::Up
        } else if i < 0 {
            UpDown::Down
        } else {
            UpDown::Middle
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Direction(pub LeftRight, pub UpDown);

pub const DIR_UP:    Direction = Direction(LeftRight::Middle, UpDown::Up);
pub const DIR_DOWN:  Direction = Direction(LeftRight::Middle, UpDown::Down);
pub const DIR_LEFT:  Direction = Direction(LeftRight::Left,   UpDown::Middle);
pub const DIR_RIGHT: Direction = Direction(LeftRight::Right,  UpDown::Middle);

impl Direction {

    fn rot_by(&self, re: i8, im: i8) -> Direction {
        let Direction(x, y) = *self;
        let (a, b) = (x as i8, y as i8);
        Direction(
            LeftRight::from_int(a * re - b * im),
            UpDown::from_int(a * im + b * re))
    }

    pub fn rot_cw(&self) -> Direction {
        self.rot_by(1, -1)
    }

    pub fn rot_ctr_cw(&self) -> Direction {
        self.rot_by(1, 1)
    }
}

#[cfg(test)]
mod tests {
    use super::Direction;
    use super::LeftRight;
    use super::UpDown;

    #[test]
    fn rotation_test() {
        let order = [
            Direction(LeftRight::Left, UpDown::Up),
            Direction(LeftRight::Middle, UpDown::Up),
            Direction(LeftRight::Right, UpDown::Up),
            Direction(LeftRight::Right, UpDown::Middle),
            Direction(LeftRight::Right, UpDown::Down),
            Direction(LeftRight::Middle, UpDown::Down),
            Direction(LeftRight::Left, UpDown::Down),
            Direction(LeftRight::Left, UpDown::Middle),
            Direction(LeftRight::Left, UpDown::Up),
        ];

        let mut it = order.iter().peekable();
        while let Some(f) = it.next() {
            if let Some(&n) = it.peek() {
                assert_eq!(f.rot_cw(), *n);
                assert_eq!(*f, n.rot_ctr_cw());
            }
        }
    }

}

