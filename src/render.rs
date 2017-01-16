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

    fn closest_polygon<'b>(origin: Vector3<f64>, direction: Vector3<f64>,
          polys: &'b Vec<Polygon>) -> Option<(&'b Polygon, Vector3<f64>, f64)> {
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
              poly.map(|(p, c)| (p, c, min))
          }

    fn lambert_contribution(point: Vector3<f64>, polys: &Vec<Polygon>,
    lights: &Vec<Vector3<f64>>, normal: Vector3<f64>) -> f64 {
        lights.iter().map(
            |&x| {
                let dir = x - point;
                let mut ndir = dir;
                ndir.2 = ndir.2 * 200.0;
                if let Some((_, _, d)) = Canvas::closest_polygon(
                    point + dir * 0.001, dir, polys) {
                        if d < dir.length() {
                            0.0
                        } else {
                            ndir.normalize().dot(normal).abs()
                        }
                    } else {
                        ndir.normalize().dot(normal).abs()
                    }
            }).sum()
    }

    fn reflect(direction: Vector3<f64>, normal: Vector3<f64>) ->
    Vector3<f64> {
        direction - normal * 2.0 * direction.dot(normal)
    }

    fn get_pixel_color(depth: u8, origin: Vector3<f64>, direction: Vector3<f64>,
        polys: &Vec<Polygon>, lights: &Vec<Vector3<f64>>) -> RGB {
            if depth == 7 {
                return RGB(0, 0, 0)
            }
            let closest = Canvas::closest_polygon(origin.into_inner(),
                                                 direction,
                                                 polys);
            if let Some((p, c, _)) = closest {
                let norm = p.normal();
                let color = p.color_at(c);
                let mut lamb = Canvas::lambert_contribution(c, polys,
                     lights, norm);
                if lamb < 0.1 {
                    lamb = 0.1
                }
                let color_with_lamb = match color {
                    RGB(r, g, b) => {
                        let mut v = Vector3(r as f64, g as f64, b as f64)
                            * lamb;
                        if v.0 > 255.0 || v.1 > 255.0 || v.2 > 255.0 {
                            match v {
                                Vector3(r, g, b) => {
                                    let max = [r, g, b].iter().map(
                                        |&x| x as i32).max().unwrap();
                                    v.0 = v.0 * 254.9 / max as f64;
                                    v.1 = v.1 * 254.9 / max as f64;
                                    v.2 = v.2 * 254.9 / max as f64;
                                }
                            }
                        }
                        RGB(v.0 as u8, v.1 as u8, v.2 as u8)
                    }
                };

                let refl_dir = Canvas::reflect(direction, norm);
                let refl_color = Canvas::get_pixel_color(
                    depth + 1, c + refl_dir * 0.01, refl_dir,
                    polys, lights);

                let color_with_refl = if refl_color.0 > 0 || refl_color.1 > 0 ||
                refl_color.2 > 0 {
                    match (color_with_lamb, refl_color) {
                        (RGB(r1, g1, b1), RGB(r2, g2, b2)) => {
                            RGB(r1 + r2 / 4, g1 + g2 / 4, b1 + b2 / 4)
                        }
                    }
                } else {
                    color_with_lamb
                };
                color_with_refl
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
                    Canvas::get_pixel_color(0, origin, dir, &poly,
                                           &scene.lighting);
            }
        }
    }
}
