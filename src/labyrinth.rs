use std::fmt;

#[derive(Clone, Copy)]
enum LeftRight {
    Left = -1,
    Middle = 0,
    Right = 1,
}

#[derive(Clone, Copy)]
enum UpDown {
    Up = -1,
    Middle = 0,
    Down = 1,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Direction((LeftRight, UpDown));

pub struct Labyrinth {
    cells: Vec<Vec<bool>>
}

impl fmt::Debug for Labyrinth {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut v = Vec::new();
        for i in self.cells.iter() {
            let s : String = i.iter().map(
                |&y| if y {'#'} else {' '}).collect();
            v.push(s);
        }
        write!(f, "{}", v.join("\n"))
    }
}

impl fmt::Display for Labyrinth {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..self.cells.len() {
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
        let v = vec![vec![false; width]; height];
        fill_labyrinth(&v);
        Labyrinth{cells : v}
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
                                 |m, _, _, v, h| m.push(Direction((h, v))));
        res
    }
}

pub struct Point {
    pub x: usize,
    pub y: usize
}

pub fn fill_labyrinth(mut v: &Vec<Vec<bool>>) {
}

