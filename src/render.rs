use geometry::*;
use std::f64;

pub struct Canvas {
    width: u16,
    height: u16,
    pixels: Vec<Vec<RGB>>,
}

impl Canvas {
    pub fn new(w: u16, h: u16) -> Canvas {
        Canvas {
            width: w,
            height: h,
            pixels: vec!(vec!(RGB(0, 0, 0); w as usize); h as usize),
        }
    }

    pub fn pixels(&self) -> &Vec<Vec<RGB>> {
        &self.pixels
    }

    fn raytrace(origin: Vector3<f64>, direction: Vector3<f64>, poly: &Polygon)->
        Option<(f64, Vector3<f64>)> {
            let (v0, v1, v2) = poly.coords;
            let poly_normal = poly.normal();

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

    fn lambert_contribution(point: Vector3<f64>, polys: &Vec<Polygon>,
    lights: &Vec<Vector3<f64>>, normal: Vector3<f64>) -> f64 {
        lights.iter().map(
            |&x| {
                let dir = *(x - point).normalize();
                if let Some(_) = Canvas::closest_polygon(
                    point + dir * 1e-6, dir, polys) {
                        0.0
                    } else {
                        dir.dot(normal)
                    }
            }).sum()
    }

    fn get_pixel_color(origin: Vector3<f64>, direction: Vector3<f64>,
        polys: &Vec<Polygon>, lights: &Vec<Vector3<f64>>) -> RGB {
            let closest = Canvas::closest_polygon(origin.into_inner(),
                                                 direction,
                                                 polys);
            if let Some((p, c, _)) = closest {
                p.color_at(c)
            } else {
                RGB(0x00, 0x00, 0x00)
            }
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
                self.pixels[b as usize][a as usize] =
                    Canvas::get_pixel_color(origin, dir, &poly,
                                           &scene.lighting);
            }
        }
    }
}
