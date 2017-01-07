/// The direction on the X axis
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum LeftRight {
    /// To the left
    Left = -1,
    /// Horizontally neutral
    Middle = 0,
    /// To the right
    Right = 1,
}

impl LeftRight {
/// Direction from 0 to a point on the X axis
///
/// Returns `Left` if a point is located to the left of 0, that is, has a
/// negative value, `Middle` if it's located at 0, and `Right` otherwise.
///
/// * `i` - the point the direction to which is to be determined.
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

/// The direction on the Y axis
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum UpDown {
    /// Downwards
    Down = -1,
    /// Vertically neutral
    Middle = 0,
    /// Upwards
    Up = 1,
}

impl UpDown {
/// Direction from 0 to a point on the Y axis
///
/// Returns `Up` if a point is located upwards of 0, `Down` if it's located
/// downwards, and `Middle` if it's at 0. The Y axis points up.
///
/// * `i` - the point the direction to which is to be determined.
///
/// # Examples
///
/// ```
/// use direction::UpDown;
/// assert_eq!(UpDown::from_int(0),   UpDown::Middle);
/// assert_eq!(UpDown::from_int(-15), UpDown::Down);
/// assert_eq!(UpDown::from_int(120), UpDown::Up);
/// ```
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
/// Two-dimensional direction.
pub struct Direction(pub LeftRight, pub UpDown);

pub const DIR_UP:    Direction = Direction(LeftRight::Middle, UpDown::Up);
pub const DIR_DOWN:  Direction = Direction(LeftRight::Middle, UpDown::Down);
pub const DIR_LEFT:  Direction = Direction(LeftRight::Left,   UpDown::Middle);
pub const DIR_RIGHT: Direction = Direction(LeftRight::Right,  UpDown::Middle);

impl Direction {

/// Multiply a direction by a complex number
///
/// Representing every direction as a pair of (-1, 0, -1), we can rotate the
/// direction if we treat this pair as a complex number and multiply it by
/// another complex number.
///
/// * `re` - the real part of the complex number to be multiplied by.
/// * `im` - the imaginary part of the complex number to be multiplied by.
///
/// # Examples
///
/// ```
/// use direction;
/// let up   = direction::DIR_UP;
/// let down = up.rot_by(-1, 0);
/// assert_eq!(down, direction::DIR_DOWN);
/// ```
    fn rot_by(&self, re: i8, im: i8) -> Direction {
        let Direction(x, y) = *self;
        let (a, b) = (x as i8, y as i8);
        Direction(
            LeftRight::from_int(a * re - b * im),
            UpDown::from_int(a * im + b * re))
    }

/// Rotate a direction clockwise by tau/8 (pi/4)
///
/// Multiply the direction by (1 - i) in order to rotate it clockwise for an
/// eight part of the full turn.
///
/// # Examples
///
/// ```
/// use direction;
/// let sw = direction::Direction(direction::LeftRight::Left,
///                               direction::UpDown::Right);
/// assert_eq!(direction::DIR_DOWN.rot_cw(), sw);
/// ```
    pub fn rot_cw(&self) -> Direction {
        self.rot_by(1, -1)
    }

/// Rotate a direction counterclockwise by tau/8 (pi/4)
///
/// Multiply the direction by (1 + i) in order to rotate it counterclockwise
/// for an eight part of the full turn.
///
/// # Examples
///
/// ```
/// use direction;
/// let se = direction::Direction(direction::LeftRight::Right,
///                               direction::UpDown::Right);
/// assert_eq!(direction::DIR_DOWN.rot_cw(), se);
/// ```
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

