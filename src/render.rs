use geometry::*;
use std::convert::Into;
use std::f64;

struct Canvas {
    width: u16,
    height: u16,
    pixels: Vec<Vec<RGB>>,
    zbuffer: Vec<Vec<u16>>,
}

impl Canvas {
    pub fn new(w: u16, h: u16) -> Canvas {
        Canvas {
            width: w,
            height: h,
            pixels: vec!(vec!(RGB(0, 0, 0); w as usize); h as usize),
            zbuffer: vec!(vec!(0; w as usize); h as usize),
        }
    }

    fn raytrace(origin: Coord3D, direction: Vector3<f64>, poly: Polygon) ->
        Option<f64> {
            let v0 : Vector3<f64> = poly.0.coords.into();
            let v1 : Vector3<f64> = poly.1.coords.into();
            let v2 : Vector3<f64> = poly.2.coords.into();
            let o  : Vector3<f64> = origin.into();
            let v0v1 = v1 - v0;
            let v0v2 = v2 - v0;
            let poly_normal = v0v1 * v0v2;
            let triangle_ray_dot = poly_normal.dot(direction);

            if triangle_ray_dot.abs() < f64::EPSILON {
                return None;
            }

            let d = poly_normal.dot(v0);
            let t = (poly_normal.dot(origin.into()) + d)/triangle_ray_dot;

            if t < 0.0 {
                return None;
            }

            let p = o + direction * t;
            let c = v0v1 * (p - v0);

            if [(v1, v0), (v2, v1), (v0, v2)].iter().any(
                |&(a, b)| poly_normal.dot((a - b) * (p - b)) < 0.0) {
                    return None;
                }

            Some(t)
        }

    pub fn render(&mut self, scene: World) {
        let origin = Coord3D(0, 0, -1);
        for a in 0..self.width {
            for b in 0..self.height {
                for v in scene.shapes.iter() {
                }
            }
        }
    }
}

