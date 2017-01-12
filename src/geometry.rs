use std::ops::{Mul,Sub,Add};
use std::convert::From;

#[derive(Clone, Copy)]
pub struct RGB (pub u8, pub u8, pub u8);

#[derive(Copy,Clone)]
pub enum Dimension {
    X,
    Y,
    Z
}

#[derive(Copy,Clone)]
pub struct Axis (pub Dimension);

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

impl<T> Vector3<T> {
    pub fn into_inner<K>(self) -> Vector3<K> where K: From<T> {
        Vector3(self.0.into(), self.1.into(), self.2.into())
    }
}

impl Vector3<f64> {
    pub fn length(&self) -> f64 {
        (self.0 * self.0 + self.1 * self.1 + self.2 * self.2).sqrt()
    }
}

pub enum Primitive {
    TriangleList,
    TriangleStrip
}

#[derive(Clone, Copy)]
pub struct Vertex {
    pub coords: Vector3<f64>,
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
    pub lighting: Vec<Vector3<f64>>,
}

pub trait Worldly {
    fn scene(&self) -> World;
}

