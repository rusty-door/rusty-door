use direction::LeftRight;
use direction::UpDown;
use direction::Direction;
use std::fmt;
use std::ops::Index;
use std::ops::IndexMut;

struct Field {
    cells: Vec<Vec<bool>>
}

impl Field {
    pub fn new(width: usize, height: usize, default: bool) -> Field {
        Field { cells: vec![vec![default; width]; height] }
    }

    fn get(&self, p: Point) -> Option<&bool> {
        self.cells.get(p.x).and_then(|k| k.get(p.y))
    }

    fn filter_around<T, F>(&self, def: bool, req: bool, p: &Point,
                           t: &mut T, mut f: F)
        where F: FnMut(&mut T, usize, usize, UpDown, LeftRight) {
            let lr = [LeftRight::Left, LeftRight::Middle, LeftRight::Right];
            let ud = [UpDown::Up, UpDown::Middle, UpDown::Down];
            for h in lr.iter() { for v in ud.iter() {
                let n = p.neighbor(Direction(*h, *v));
                let Point{x, y} = n;
                if req == *self.get(n).unwrap_or(&def) {
                    f(t, x, y, *v, *h);
                }
            } }
        }
}

impl Index<Point> for Field {
    type Output = bool;
    fn index(&self, index: Point) -> &bool {
        self.get(index).expect("Out of bounds")
    }
}

impl IndexMut<Point> for Field {
    fn index_mut(&mut self, index: Point) -> &mut bool {
        self.cells.index_mut(index.x).index_mut(index.y)
    }
}

impl fmt::Debug for Field {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut v = Vec::new();
        for i in self.cells.iter().rev() {
            let s : String = i.iter().map(
                |&y| if y {'#'} else {' '}).collect();
            v.push(s);
        }
        write!(f, "{}", v.join("\n"))
    }
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in (0..self.cells.len()).rev() {
            let len = self.cells[i].len();
            for j in 0..len {
                try!(write!(f, "{}", if self.cells[i][j] {'#'} else {' '}));
            }
            if i != len - 1 {
                try!(write!(f, "\n"));
            }
        }
        Ok(())
    }
}

pub struct Labyrinth(Field);

impl fmt::Debug for Labyrinth {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl fmt::Display for Labyrinth {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Labyrinth {
    pub fn new(width: usize, height: usize) -> Labyrinth {
        let mut v = Labyrinth(Field::new(width, height, true));
        v.fill_labyrinth();
        v
    }

    fn fill_labyrinth(&mut self) {
        let mut visited = Field::new(
            self.0.cells[0].len(), self.0.cells.len(), false);
        let mut stack = vec![Point{x : 1, y : 1}];
        let mut dir = Direction(LeftRight::Middle, UpDown::Down);
        let mut walked = 0;
        'main: while let Some(f) = stack.pop() {
            let Point{x, y} = f;
            visited[f.clone()] = true;

            if self.essential_cell(&f) {
                continue;
            }

            self.0.cells[x][y] = false;

            let good_neighbors = |m : &mut Vec<Point>, vidx, hidx, v, h| {
                if v == UpDown::Middle || h == LeftRight::Middle {
                    m.push(Point{x : vidx, y : hidx})
                } };

            visited.filter_around(true, false, &f, &mut stack, good_neighbors);
        }
    }

    fn essential_cell(&self, p: &Point) -> bool {
        let en = self.empty_neighbors(&p);
        let in_hdir = |dir, &Direction(hdir, _)| hdir == dir;
        let in_vdir = |dir, &Direction(_, vdir)| vdir == dir;

        en.iter().any(|x| in_hdir(LeftRight::Left, x)) &&
            en.iter().any(|x| in_hdir(LeftRight::Right, x)) ||
            en.iter().any(|x| in_vdir(UpDown::Up, x)) &&
            en.iter().any(|x| in_vdir(UpDown::Down, x))
    }

    fn empty_neighbors(&self, p: &Point) -> Vec<Direction> {
        let mut res = Vec::new();
        self.0.filter_around(false, false, p, &mut res,
                                 |m, _, _, v, h| m.push(Direction(h, v)));
        res
    }
}

#[derive(Copy, Clone)]
pub struct Point {
    pub x: usize,
    pub y: usize
}

impl Point {
    fn neighbor(&self, d: Direction) -> Point {
        let Point{x, y} = *self;
        let Direction(h, v) = d;
        let hidx = (y as i32 + h as i32) as usize;
        let vidx = (x as i32 + v as i32) as usize;
        Point{x: vidx, y: hidx}
    }
}

