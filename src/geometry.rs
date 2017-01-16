use std::ops::{Index,IndexMut,Mul,Sub,Add};
use std::convert::From;
use std::f64::consts::PI;
use std::cmp::min;

#[derive(Clone, Copy, Debug)]
pub struct RGB (pub u8, pub u8, pub u8);

#[derive(Copy,Clone)]
pub enum Dimension {
    X,
    Y,
    Z
}

#[derive(Copy,Clone)]
pub struct Plane (pub Dimension);

#[derive(Copy,Clone,Debug)]
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

impl<T> Index<Dimension> for Vector3<T> {
    type Output = T;
    fn index(&self, index: Dimension) -> &T {
        match index {
            Dimension::X => &self.0,
            Dimension::Y => &self.1,
            Dimension::Z => &self.2,
        }
    }
}

impl<T> IndexMut<Dimension> for Vector3<T> {
    fn index_mut(&mut self, index: Dimension) -> &mut T {
        match index {
            Dimension::X => &mut self.0,
            Dimension::Y => &mut self.1,
            Dimension::Z => &mut self.2,
        }
    }
}

impl<T: Copy + Mul<T, Output=T>> Mul<T> for Vector3<T> {
    type Output = Vector3<T>;
    fn mul(self, rhs: T) -> Vector3<T> {
        Vector3(self.0 * rhs, self.1 * rhs, self.2 * rhs)
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

impl<T> Into<(T, T, T)> for Vector3<T> {
    fn into(self) -> (T, T, T) {
        (self.0, self.1, self.2)
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

impl<T: Mul<T, Output=T> + Add<T, Output=T> + Copy> Vector3<T> {
    pub fn dot(&self, rhs: Vector3<T>) -> T {
        self.0 * rhs.0 + self.1 * rhs.1 + self.2 * rhs.2
    }

}

impl Vector3<f64> {
    pub fn length(&self) -> f64 {
        (self.0 * self.0 + self.1 * self.1 + self.2 * self.2).sqrt()
    }

    pub fn normalize<'b>(&self) -> Self {
        let len = self.length();
        let mut v = *self;
        v.0 = v.0 / len; v.1 = v.1 / len; v.2 = v.2 / len;
        v
    }
}

impl<T> Vector3<T> {
    pub fn into_inner<K>(self) -> Vector3<K> where K: From<T> {
        Vector3(self.0.into(), self.1.into(), self.2.into())
    }
}

pub enum Primitive {
    TriangleList,
    TriangleStrip
}

#[derive(Clone)]
pub struct Texture2d {
    pub width: u16,
    pub height: u16,
    pub values: Vec<RGB>
}

#[derive(Clone)]
pub enum ColorGenerator {
    Uniform(RGB),
    Linear(RGB, RGB, RGB),
    SphereTexture(Vector3<f64>, Texture2d),
    PlaneTexture(Vector3<f64>, Vector3<f64>, Plane, Texture2d),
}

#[derive(Clone)]
pub struct Material {
    pub color: ColorGenerator
}

#[derive(Clone)]
pub struct Polygon {
    pub coords: (Vector3<f64>, Vector3<f64>, Vector3<f64>),
    pub material: Material,
}

impl Polygon {
    pub fn color_at(&self, point: Vector3<f64>) -> RGB {
        match &self.material.color {
            &ColorGenerator::Uniform(rgb) => rgb,
            &ColorGenerator::Linear(c1, c2, c3) => {
                let dists : Vec<f64> =
                    [self.coords.0, self.coords.1, self.coords.2].
                    iter().map(|&x| (x - point).length()).collect();
                let t : f64 = dists.iter().sum();
                let colors : Vec<(RGB, f64)> = [c1, c2, c3].iter().zip(
                    dists.iter()).map(|(&x, &y)| (x, y)).collect();

                let r : f64 = colors.iter().map(|&(c, d)| 1.0-d*c.0 as f64 / t).sum();
                let g : f64 = colors.iter().map(|&(c, d)| 1.0-d*c.1 as f64 / t).sum();
                let b : f64 = colors.iter().map(|&(c, d)| 1.0-d*c.2 as f64 / t).sum();

                RGB(r as u8, g as u8, b as u8)
            },
            &ColorGenerator::SphereTexture(center, ref texture) => {
                let mut dir = point - center;
                dir.2 = 100.0 * dir.2;
                let dir = dir.normalize();
                let tu  = dir.0.asin() / PI + 0.5;
                let tv  = dir.2.asin() / PI + 0.5;
                let x = min((tu * texture.width as f64) as usize,
                                    texture.width as usize - 1);
                let y = min((tv * texture.height as f64) as usize,
                                    texture.height as usize - 1);
                texture.values[y * texture.width as usize + x]
            },
            &ColorGenerator::PlaneTexture(start, end, Plane(d), ref tex) => {
                let g = |p: &Vector3<f64>| match d {
                    Dimension::X => (p.1, p.2),
                    Dimension::Y => (p.0, p.2),
                    Dimension::Z => (p.0, p.1),
                };

                let (c1, c2) = g(&point);
                let (s1, s2) = g(&start);
                let (e1, e2) = g(&end);

                let x = (c1 - s1) / (e1 - s1);
                let y = (c2 - s2) / (e2 - s2);

                if x >= tex.width as f64 || y >= tex.height as f64 ||
                x < 0.0 || y < 0.0 {
                    RGB(0, 0, 0)
                } else {
                    tex.values[y as usize * tex.width as usize + x as usize]
                }
            }
        }
    }

    pub fn normal(&self) -> Vector3<f64> {
        let (v0, v1, v2) = self.coords;
        let v0v1 = v1 - v0;
        let v0v2 = v2 - v0;
        let mut poly_normal = v0v1 * v0v2;
        poly_normal.normalize()
    }
}

pub struct Shape {
    pub verts: Vec<Vector3<f64>>,
    pub material: Material,
    pub primitive: Primitive,
}

impl Shape {
    pub fn to_polygons(&self) -> Vec<Polygon> {
        let g = |x : &[Vector3<f64>]| Polygon {
            coords : (x[0], x[1], x[2]),
            material : self.material.clone(),
        };
        match self.primitive {
            Primitive::TriangleList =>  self.verts.chunks (3).map(
                g).collect(),
            Primitive::TriangleStrip => self.verts.windows(3).map(
                g).collect()
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

