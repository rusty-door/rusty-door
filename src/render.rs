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

    fn raytrace(origin: Coord3D, direction: Vector3<f64>, poly: &Polygon) ->
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

#[cfg(test)]
mod tests {
    use geometry::*;
    use super::Canvas;

    #[test]
    fn raytrace_test() {
        let origin = Coord3D(1, 1, 1);
        let dir : Vector3<f64> = Vector3(-0.125, 1.0, 0.5);
        let poly = Polygon (
            Vertex { coords: Coord3D(-1, 8 , 3), color: RGB(0, 0, 0) },
            Vertex { coords: Coord3D( 1, 10, 2), color: RGB(0, 0, 0) },
            Vertex { coords: Coord3D( 0, 9 , 5), color: RGB(0, 0, 0) });

        // Valid intersection
        assert!(Canvas::raytrace(origin, dir, &poly).is_some());

        let another_origin = Coord3D(100, 100, 100);

        // Ray is outside of the triangle
        assert!(Canvas::raytrace(another_origin, dir, &poly).is_none());

        let another_poly = Polygon (
            Vertex { coords: Coord3D(-5, -10, -2), color: RGB(0, 0, 0) },
            Vertex { coords: Coord3D( 5, -12, -1), color: RGB(0, 0, 0) },
            Vertex { coords: Coord3D( 4, -5 , -6), color: RGB(0, 0, 0) });

        // Ray intersects the triangle behind the origin
        assert!(Canvas::raytrace(origin, dir, &another_poly).is_none());

        let another_direction : Vector3<f64> = Vector3(1.0, 0.0, 0.0);

        let third_poly = Polygon (
            Vertex { coords: Coord3D( 5, 4, -2), color: RGB(0, 0, 0) },
            Vertex { coords: Coord3D( 2, 3, -2), color: RGB(0, 0, 0) },
            Vertex { coords: Coord3D( 4, 1, -2), color: RGB(0, 0, 0) });

        // Ray is parallel to the triangle
        assert!(Canvas::raytrace(origin, another_direction,
                                 &third_poly).is_none());
    }
}

