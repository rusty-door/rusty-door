use std::ops::Index;
use std::convert::From;

#[derive(Clone, Copy)]
pub struct RGB (pub u8, pub u8, pub u8);

pub struct Angle (f64);

impl Angle {
    pub fn cos(&self) -> f64 {
        self.0.cos()
    }

    pub fn sin(&self) -> f64 {
        self.0.sin()
    }
}

#[derive(Copy,Clone)]
pub enum Dimension {
    X,
    Y,
    Z
}

#[derive(Copy,Clone)]
pub struct Coord2D (i32, i32);

impl Coord2D {
    fn rotate(&self, a: Angle) -> Coord2D {
        Coord2D(self.0 * (a.cos() as i32) - self.1 * (a.sin() as i32),
                self.0 * (a.sin() as i32) + self.1 * (a.cos() as i32))
    }

    fn to_space(&self, plane: Plane, val: i32) -> Coord3D {
        match plane.0 {
            Dimension::X => Coord3D(val, self.0, self.1),
            Dimension::Y => Coord3D(self.0, val, self.1),
            Dimension::Z => Coord3D(self.0, self.1, val)
        }
    }
}

#[derive(Copy,Clone)]
pub struct Axis  (Dimension);

#[derive(Copy,Clone)]
pub struct Plane (Dimension);

#[derive(Copy,Clone)]
pub struct Coord3D (pub i32, pub i32, pub i32);

impl Index<Axis> for Coord3D {
    type Output = i32;
    fn index(&self, index: Axis) -> &i32 {
        match index.0 {
            Dimension::X => &self.0,
            Dimension::Y => &self.1,
            Dimension::Z => &self.2
        }
    }
}

impl Coord3D {
    fn at_plane(&self, plane: Plane) -> Coord2D {
        match plane.0 {
            Dimension::X => Coord2D(self.1, self.2),
            Dimension::Y => Coord2D(self.0, self.2),
            Dimension::Z => Coord2D(self.0, self.1)
        }
    }

    fn rotate(&self, plane: Plane, a: Angle) -> Coord3D {
        self.at_plane(plane).rotate(a).to_space(plane, self[Axis(plane.0)])
    }
}

pub enum Primitive {
    TriangleList,
    TriangleStrip
}

#[derive(Clone, Copy)]
pub struct Vertex {
    pub coords: Coord3D,
    pub color: RGB,
}

pub struct Polygon ([Vertex; 3]);

pub struct Shape {
    pub verts: Vec<Vertex>,
    pub primitive: Primitive,
}

impl Into<Vec<Polygon>> for Shape {
    fn into(self: Shape) -> Vec<Polygon> {
        match self.primitive {
            Primitive::TriangleList =>  self.verts.chunks (3).map(|x|
                                 Polygon([x[0], x[1], x[2]])).collect(),
            Primitive::TriangleStrip => self.verts.windows(3).map(|x|
                                 Polygon([x[0], x[1], x[2]])).collect()
        }
    }
}

pub struct World {
    pub shapes: Vec<Shape>,
    pub lighting: Vec<Coord3D>,
}

pub trait Worldly {
    fn scene(&self) -> World;
}

