use std::ops::{Mul,Sub,Add};
use std::convert::From;

#[derive(Clone, Copy)]
pub struct RGB (pub u8, pub u8, pub u8);

#[derive(Copy,Clone)]
pub struct Coord2D (i32, i32);

#[derive(Copy,Clone)]
pub struct Vector3<T> (pub T, pub T, pub T);

impl<T: Copy + Mul<T, Output=T> + Sub<T, Output=T>>
Mul<Vector3<T>> for Vector3<T> {
    type Output = Vector3<T>;
    fn mul(self, rhs: Vector3<T>) -> Vector3<T> {
        Vector3(self.1 * rhs.2 - self.2 * rhs.1,
                self.2 * rhs.0 - self.0 * rhs.2,
                self.0 * rhs.1 - self.1 * rhs.0)
    }
}

impl<T: Copy + Mul<T, Output=T>> Mul<T> for Vector3<T> {
    type Output = Vector3<T>;
    fn mul(self, rhs: T) -> Vector3<T> {
        Vector3(rhs * self.0, rhs * self.1, rhs * self.2)
    }
}

impl<T: Add<T, Output=T>> Add<Vector3<T>> for Vector3<T> {
    type Output = Vector3<T>;
    fn add(self, rhs: Vector3<T>) -> Vector3<T> {
        Vector3(self.0 + rhs.0,
                self.1 + rhs.1,
                self.2 + rhs.2)
    }
}

impl<T: Sub<T, Output=T>> Sub<Vector3<T>> for Vector3<T> {
    type Output = Vector3<T>;
    fn sub(self, rhs: Vector3<T>) -> Vector3<T> {
        Vector3(self.0 - rhs.0,
                self.1 - rhs.1,
                self.2 - rhs.2)
    }
}

impl<T: Mul<T, Output=T> + Add<T, Output=T>> Vector3<T> {
    pub fn dot(self, rhs: Vector3<T>) -> T {
        self.0 * rhs.0 + self.1 * rhs.1 + self.2 * rhs.2
    }
}

impl Vector3<f64> {
    pub fn length(&self) -> f64 {
        (self.0 * self.0 + self.1 * self.1 + self.2 * self.2).sqrt()
    }
}

#[derive(Copy,Clone)]
pub struct Coord3D (pub i32, pub i32, pub i32);

impl<T: From<i32>> Into<Vector3<T>> for Coord3D {
    fn into(self) -> Vector3<T> {
        Vector3(self.0.into(), self.1.into(), self.2.into())
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

#[derive(Clone, Copy)]
pub struct Polygon (pub Vertex, pub Vertex, pub Vertex);

pub struct Shape {
    pub verts: Vec<Vertex>,
    pub primitive: Primitive,
}

impl Shape {
    pub fn to_polygons(&self) -> Vec<Polygon> {
        match self.primitive {
            Primitive::TriangleList =>  self.verts.chunks (3).map(|x|
                                 Polygon(x[0], x[1], x[2])).collect(),
            Primitive::TriangleStrip => self.verts.windows(3).map(|x|
                                 Polygon(x[0], x[1], x[2])).collect()
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

