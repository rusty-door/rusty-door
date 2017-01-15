use geometry::*;
use std::convert::Into;
use std::f64;
use std::cmp::Ordering;

pub struct Canvas {
    width: u16,
    height: u16,
    pixels: Vec<Vec<RGB>>,
    zbuffer: Vec<Vec<f64>>,
}

impl Canvas {
    pub fn new(w: u16, h: u16) -> Canvas {
        Canvas {
            width: w,
            height: h,
            pixels: vec!(vec!(RGB(0, 0, 0); w as usize); h as usize),
            zbuffer: vec!(vec!(0.0; w as usize); h as usize),
        }
    }

    pub fn pixels(&self) -> &Vec<Vec<RGB>> {
        &self.pixels
    }

    fn raytrace(origin: Vector3<f64>, direction: Vector3<f64>, poly: &Polygon)->
        Option<(f64, Vector3<f64>)> {
            let (v0, v1, v2) = poly.coords;
            let v0v1 = v1 - v0;
            let v0v2 = v2 - v0;
            let poly_normal = v0v1 * v0v2;
            let triangle_ray_dot = poly_normal.dot(direction);

            if triangle_ray_dot.abs() < f64::EPSILON {
                return None;
            }

            let t = (poly_normal.dot(v0 - origin))/triangle_ray_dot;

            if t < 0.0 {
                return None;
            }

            let p = origin + direction * t;

            if [(v1, v0), (v2, v1), (v0, v2)].iter().any(
                |&(a, b)| poly_normal.dot((a - b) * (p - b)) < 0.0) {
                    return None;
                }

            Some((t, p))
        }

    fn closest_polygon(origin: Vector3<f64>, direction: Vector3<f64>,
          polys: &Vec<Polygon>) -> Option<(Polygon, Vector3<f64>, f64)> {
              let mut min = f64::INFINITY;
              let mut poly = None;
              for p in polys {
                  if let Some((t, c)) = Canvas::raytrace(
                      origin, direction, p) {
                          if t < min {
                              min = t;
                              poly = Some((p, c));
                          }
                      }
              }
              poly.map(|(&p, c)| (p, c, min))
          }

    pub fn render(&mut self, scene: &World) {
        let origin = Vector3(self.width  as f64 / 2.0,
                             self.height as f64 / 2.0,
                             -1.0);
        let poly = scene.shapes.iter().flat_map(|x| x.to_polygons()).collect();
        for a in 0..self.width {
            for b in 0..self.height {
                let dir : Vector3<f64> = Vector3(
                    (a as f64 - self.width  as f64 / 2.0),
                    (b as f64 - self.height as f64 / 2.0),
                    1.0);
                let closest = Canvas::closest_polygon(origin.into_inner(),
                                                     dir,
                                                     &poly);
                if let Some((p, c, d)) = closest {
                    self.pixels[b as usize][a as usize] = p.color_at(c);
                    self.zbuffer[b as usize][a as usize] = d;
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
        let origin = Vector3(1, 1, 1).into_inner();
        let dir : Vector3<f64> = Vector3(-0.125, 1.0, 0.5);
        let poly = Polygon (
            Vertex { coords: Vector3(-1.0, 8.0 , 3.0), color: RGB(0, 0, 0) },
            Vertex { coords: Vector3( 1.0, 10.0, 2.0), color: RGB(0, 0, 0) },
            Vertex { coords: Vector3( 0.0, 9.0 , 5.0), color: RGB(0, 0, 0) });

        // Valid intersection
        assert!(Canvas::raytrace(origin, dir, &poly).is_some());

        let another_origin = Vector3(100, 100, 100).into_inner();

        // Ray is outside of the triangle
        assert!(Canvas::raytrace(another_origin, dir, &poly).is_none());

        let another_poly = Polygon (
            Vertex { coords: Vector3(-5.0, -10.0, -2.0), color: RGB(0, 0, 0) },
            Vertex { coords: Vector3( 5.0, -12.0, -1.0), color: RGB(0, 0, 0) },
            Vertex { coords: Vector3( 4.0, -5.0 , -6.0), color: RGB(0, 0, 0) });

        // Ray intersects the triangle behind the origin
        assert!(Canvas::raytrace(origin, dir, &another_poly).is_none());

        let another_direction : Vector3<f64> = Vector3(1.0, 0.0, 0.0);

        let third_poly = Polygon (
            Vertex { coords: Vector3( 5.0, 4.0, -2.0), color: RGB(0, 0, 0) },
            Vertex { coords: Vector3( 2.0, 3.0, -2.0), color: RGB(0, 0, 0) },
            Vertex { coords: Vector3( 4.0, 1.0, -2.0), color: RGB(0, 0, 0) });

        // Ray is parallel to the triangle
        assert!(Canvas::raytrace(origin, another_direction,
                                 &third_poly).is_none());
    }
}

