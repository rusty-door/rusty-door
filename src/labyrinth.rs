use direction::LeftRight;
use direction::UpDown;
use direction::Direction;
use std::fmt;
use std::ops::Index;
use std::ops::IndexMut;
use std::collections::LinkedList;

struct Field {
    cells: Vec<Vec<bool>>,
    height: usize,
    width: usize
}

impl Field {
    pub fn new(width: usize, height: usize, default: bool) -> Field {
        Field {
            cells: vec![vec![default; width]; height],
            height: height,
            width: width,
        }
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
                if req == *self.get(n).unwrap_or(&def) {
                    f(t, n.x, n.y, *v, *h);
                }
            } }
        }

    fn get_wall_character(&self, p: Point) -> char {
        let Point{x: i, y: j} = p;
        if ! self.cells[i][j] {
            '.'
        } else if j > 0 && *self.get(Point{x: i, y: j-1}).unwrap_or(&false) {
            if i > 0 && *self.get(Point{x: i-1, y: j}).unwrap_or(&false) {
                if *self.get(Point{x: i, y: j+1}).unwrap_or(&false) {
                    if *self.get(Point{x: i+1, y: j}).unwrap_or(&false) {
                        0x6e as char
                    } else {
                        0x77 as char
                    }
                } else if *self.get(Point{x: i+1, y: j}).unwrap_or(&false) {
                    0x75 as char
                } else {
                    0x6B as char
                }

            } else if *self.get(Point{x: i, y: j+1}).unwrap_or(&false) {
                if *self.get(Point{x: i+1, y: j}).unwrap_or(&false) {
                    0x76 as char
                } else {
                    0x71 as char
                }
            } else if *self.get(Point{x: i+1, y: j}).unwrap_or(&false) {
                0x6A as char
            } else {
                0x71 as char
            }

        } else {
            if i > 0 && *self.get(Point{x: i-1, y: j}).unwrap_or(&false) {
                if *self.get(Point{x: i, y: j+1}).unwrap_or(&false) {
                    if *self.get(Point{x: i+1, y: j}).unwrap_or(&false) {
                        0x74 as char
                    } else {
                        0x6C as char
                    }
                } else if *self.get(Point{x: i+1, y: j}).unwrap_or(&false) {
                    0x78 as char
                } else {
                    0x78 as char
                }

            } else if *self.get(Point{x: i, y: j+1}).unwrap_or(&false) {
                if *self.get(Point{x: i+1, y: j}).unwrap_or(&false) {
                    0x6D as char
                } else {
                    0x71 as char
                }
            } else if *self.get(Point{x: i+1, y: j}).unwrap_or(&false) {
                0x78 as char
            } else {
                '#'
            }
        }
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
        for i in (0..self.height).rev() {
            for j in 0..self.width {
                if ! *self.get(Point{x: i, y: j}).unwrap_or(&false) {
                    try!(write!(f, "."));
                } else {
                    try!(write!(f, "\x1b(0{}\x1b(B", self.get_wall_character(
                                        Point{x: i, y: j})));
                }
            }
            if i != self.width - 1 {
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
    pub fn new(width: usize, height: usize, seed: u16) -> Labyrinth {
        let mut v = Labyrinth(Field::new(width, height, true));
        v.fill_labyrinth(seed);
        v
    }

    fn fill_labyrinth(&mut self, seed: u16) {
        let mut visited = Field::new(self.0.width, self.0.height, false);
        let mut stack = LinkedList::new();
        stack.push_back(Point{x : 1, y : 1});
        let mut dir = Direction(LeftRight::Middle, UpDown::Up);
        let mut prng : u16 = 0;
        let mut total = 0;
        'main: while let Some(f) = stack.pop_front() {
            let mut curr = f;
            'run: loop {
                if visited[curr] {
                    continue 'main;
                }

                visited[curr] = true;

                if self.essential_cell(&curr) {
                    continue 'main;
                }

                total = total + 1;
                self.0[curr] = false;

                let good_neighbors = |m : &mut LinkedList<Point>, vidx, hidx,
                v, h| { if v == UpDown::Middle || h == LeftRight::Middle {
                    m.push_back(Point{x : vidx, y : hidx})
                } };

                visited.filter_around(true, false, &curr, &mut stack,
                                      good_neighbors);

                let prng_in = (prng as u32 * 0x302 * (seed as u32 * 2 + 1))
                    as u16;
                let rot_dir = |x: Direction| if total * prng_in as u32
                    % (1 << 16) > (1 << 15) { x.rot_cw().rot_cw() }
                else { x.rot_ctr_cw().rot_ctr_cw() };

                for _ in 0 .. total * ((prng_in as u32 % 2) * 2 + 1) % 4 {
                    dir = rot_dir(dir);
                }
                prng = prng + 1;

                for _ in 0 .. 4 {
                    let cp = curr.neighbor(dir);
                    if *self.0.get(cp).unwrap_or(&false) &&
                        ! visited[cp] && ! self.essential_cell(&cp) {
                            curr = cp;
                            continue 'run;
                        } else {
                            dir = rot_dir(dir);
                        }
                }
                continue 'main;
            }
        }
    }

    fn essential_cell(&self, p: &Point) -> bool {
        let en = self.empty_neighbors(&p);
        let in_hdir = |dir, &Direction(hdir, _)| hdir == dir;
        let in_vdir = |dir, &Direction(_, vdir)| vdir == dir;

        ! (en.iter().all(|x| in_hdir(LeftRight::Left, x)) ||
           en.iter().all(|x| in_hdir(LeftRight::Right, x)) ||
           en.iter().all(|x| in_vdir(UpDown::Up, x)) ||
           en.iter().all(|x| in_vdir(UpDown::Down, x))) &&
            (en.iter().any(|x| in_hdir(LeftRight::Left, x)) &&
             en.iter().any(|x| in_hdir(LeftRight::Right, x)) ||
             en.iter().any(|x| in_vdir(UpDown::Up, x)) &&
             en.iter().any(|x| in_vdir(UpDown::Down, x)) ||
             en.iter().any(|x| in_hdir(LeftRight::Middle, x)) &&
             en.iter().any(|x| in_vdir(UpDown::Middle, x)))
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
        Point{
            x: (self.x as i32 + d.1 as i32) as usize,
            y: (self.y as i32 + d.0 as i32) as usize
        }
    }
}

