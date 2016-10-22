use direction::LeftRight;
use direction::UpDown;
use direction::Direction;
use std::fmt;

pub struct Labyrinth {
    cells: Vec<Vec<bool>>
}

impl fmt::Debug for Labyrinth {
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

impl fmt::Display for Labyrinth {
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

impl Labyrinth {
    pub fn new(width: usize, height: usize) -> Labyrinth {
        let mut v = Labyrinth{cells : vec![vec![true; width]; height]};
        v.fill_labyrinth();
        v
    }

    fn fill_labyrinth(&mut self) {
        let mut visited =
            vec![vec![false; self.cells[0].len()]; self.cells.len()];
        let mut stack = vec![Point{x : 1, y : 1}];
        'main: while let Some(f) = stack.pop() {
            let Point{x, y} = f;
            visited[x][y] = true;

            if self.essential_cell(&f) {
                continue;
            }

            self.cells[x][y] = false;

            let good_neighbors = |m : &mut Vec<Point>, vidx, hidx, v, h| {
                if v == UpDown::Middle || h == LeftRight::Middle {
                    m.push(Point{x : vidx, y : hidx})
                } };

            Labyrinth::filter_around(&visited, true, false, &f, &mut stack,
                                     good_neighbors);
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

    fn filter_around<T, F>(c: &Vec<Vec<bool>>, def: bool, req: bool, p: &Point,
                        t: &mut T, mut f: F)
        where F: FnMut(&mut T, usize, usize, UpDown, LeftRight) {
            let Point{x, y} = *p;
            let e = vec![def; c[x].len()];
            let lr = [LeftRight::Left, LeftRight::Middle, LeftRight::Right];
            let ud = [UpDown::Up, UpDown::Middle, UpDown::Down];
            for h in lr.iter() { for v in ud.iter() {
                let hidx = (y as i32 + *h as i32) as usize;
                let vidx = (x as i32 + *v as i32) as usize;
                if !req^*c.get(vidx).unwrap_or(&e).get(hidx).unwrap_or(&def) {
                    f(t, vidx, hidx, *v, *h);
                }
            } }
        }

    fn empty_neighbors(&self, p: &Point) -> Vec<Direction> {
        let mut res = Vec::new();
        Labyrinth::filter_around(&self.cells, false, false, p, &mut res,
                                 |m, _, _, v, h| m.push(Direction(h, v)));
        res
    }
}

pub struct Point {
    pub x: usize,
    pub y: usize
}

