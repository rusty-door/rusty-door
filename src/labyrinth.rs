use std::fmt;

pub struct Labyrinth {
    cells: Vec<Vec<bool>>
}

impl fmt::Debug for Labyrinth {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut v = Vec::new();
        for i in self.cells.iter() {
            let s : String = i.iter().map(
                |&y| if y {' '} else {'#'}).collect();
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
                try!(write!(f, "{}", if self.cells[i][j] {' '} else {'#'}));
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
}

pub struct Point {
    pub x: usize,
    pub y: usize
}

pub fn fill_labyrinth(mut v: &Vec<Vec<bool>>) {
}

