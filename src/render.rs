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

struct Voxel<'b> {
    polygons: Vec<&'b Polygon>
}

impl<'b> Default for Voxel<'b> {
    fn default() -> Voxel<'b> {
        Voxel {
            polygons: vec!()
        }
    }
}

struct Space<'b> {
    grid: [[[Voxel<'b>; 12]; 12]; 12]
}

impl<'b> Space<'b> {
    fn new(poly: &'b Vec<Polygon>) -> Space<'b> {
        let mut g : [[[Voxel; 12]; 12]; 12] = Default::default();
        for p in poly {
            let verts = [p.0, p.1, p.2];
            let coords : Vec<&Vector3<f64>> = verts.iter().map(
                |x| &x.coords).collect();

            let cmp = |x: &f64, y: &f64| -> Ordering {
                if *x < *y {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            };

            let mut xs : Vec<f64> = coords.iter().map(|c| c.0).collect();
            xs.sort_by(&cmp);
            let mut ys : Vec<f64> = coords.iter().map(|c| c.1).collect();
            ys.sort_by(&cmp);
            let mut zs : Vec<f64> = coords.iter().map(|c| c.2).collect();
            zs.sort_by(&cmp);

            let range = |a: &Vec<f64>, d: Dimension| {
                match (Space::dimension_idx(a[0], d),
                       Space::dimension_idx(a[2], d)) {
                    (Some(left), Some(right)) => left..(right + 1),
                    _ => 0..0
                }
            };

            for x in range(&xs, Dimension::X) {
                for y in range(&ys, Dimension::Y) {
                    for z in range(&zs, Dimension::Z) {
                        g[x][y][z].polygons.push(p);
                    }
                }
            }
        }
        Space { grid: g }
    }

    fn dimension_idx(a: f64, d: Dimension) -> Option<usize> {
        let ar = match d {
            Dimension::X => (0.0, 640.0),
            Dimension::Y => (0.0, 480.0),
            Dimension::Z => (0.0, 10.0),
        };
        if a >= ar.1 || a < ar.0 {
            None
        } else {
            Some(((a - ar.0) * 12 as f64 / (ar.1 - ar.0)).floor() as usize)
        }
    }

    fn ray_boxes(&self, origin: Vector3<f64>, direction: Vector3<f64>) ->
    Vec<(usize, usize, usize)> {
        let mut point = origin;

        let inv_dir = Vector3(
            1.0 / direction[Dimension::X],
            1.0 / direction[Dimension::Y],
            1.0 / direction[Dimension::Z]);

        let dir_sign = Vector3(
            if direction[Dimension::X] > 0.0 { 1 } else { -1 },
            if direction[Dimension::Y] > 0.0 { 1 } else { -1 },
            if direction[Dimension::Z] > 0.0 { 1 } else { -1 },
        );

        let dims = [Dimension::X, Dimension::Y, Dimension::Z];
        for &d in dims.iter() {
            if point[d] < 0.0 {
                point = point - direction * inv_dir[d] * point[d];
            }
        }

        let mut coords = Vector3(
            Space::dimension_idx(point[Dimension::X], Dimension::X).
                unwrap() as i32,
            Space::dimension_idx(point[Dimension::Y], Dimension::Y).
                unwrap() as i32,
            Space::dimension_idx(point[Dimension::Z], Dimension::Z).
                unwrap() as i32);

        let nearest_bound = |c: &Vector3<i32>, d: Dimension| {
            (c[d] + dir_sign[d]) as f64 * match d {
                Dimension::X => 640.0,
                Dimension::Y => 480.0,
                Dimension::Z =>  10.0
            } / 12.0
        };

        let mut t_max = Vector3(
                (nearest_bound(&coords, Dimension::X) - point[Dimension::X]) * inv_dir[Dimension::X],
                (nearest_bound(&coords, Dimension::Y) - point[Dimension::Y]) * inv_dir[Dimension::Y],
                (nearest_bound(&coords, Dimension::Z) - point[Dimension::Z]) * inv_dir[Dimension::Z],
            );

        let t_delta = Vector3(640.0 / 12 as f64 * inv_dir[Dimension::X],
                              480.0 / 12 as f64 * inv_dir[Dimension::Y],
                               10.0 / 12 as f64 * inv_dir[Dimension::Z]);

        let coords_usize = (coords.0 as usize,
                            coords.1 as usize,
                            coords.2 as usize);

        let mut result = vec!(coords_usize);
        loop {
            let mut dims_vec = dims.to_vec();
            dims_vec.sort_by(|&a, &b| {
                        if t_max[a] < t_max[b] {
                            Ordering::Less
                        } else {
                            Ordering::Greater
                        }});
            let d = dims_vec[0];
            coords[d] = coords[d] + dir_sign[d];
            if coords[d] < 0 || coords[d] >= 12 {
                break;
            }

            let coords_usize = (coords.0 as usize,
                                coords.1 as usize,
                                coords.2 as usize);
            result.push(coords_usize);

            point = point + direction * t_max[d];
            t_max = Vector3(
                (nearest_bound(&coords, Dimension::X) - point[Dimension::X]) * inv_dir[Dimension::X],
                (nearest_bound(&coords, Dimension::Y) - point[Dimension::Y]) * inv_dir[Dimension::Y],
                (nearest_bound(&coords, Dimension::Z) - point[Dimension::Z]) * inv_dir[Dimension::Z],
            );
        }

        result
    }
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
            let v0 : Vector3<f64> = poly.0.coords.into();
            let v1 : Vector3<f64> = poly.1.coords.into();
            let v2 : Vector3<f64> = poly.2.coords.into();
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

    fn closest_polygon<'b>(origin: Vector3<f64>, direction: Vector3<f64>,
          space: &Space<'b>) -> Option<(Polygon, Vector3<f64>, f64)> {
              let boxes = space.ray_boxes(origin, direction);
              for (x, y, z) in boxes {
                  let polys = space.grid[x][y][z].polygons.clone();
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
                  if let Some((&p, c)) = poly {
                      return Some((p, c, min));
                  }
              }
              return None;
/*                  let polys : Vec<&'b Polygon> = space.grid.iter().flat_map(
                      |x| -> Vec<&'b Polygon> {
                          x.iter().flat_map(
                              |y| -> Vec<&'b Polygon> {
                                  y.iter().flat_map(
                                      |z| -> Vec<&'b Polygon> {
                                          z.polygons.clone()
                                      }).collect()
                              }).collect()
                      }).collect();

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
                  if let Some((&p, c)) = poly {
                      println!("Get at {:?} -> {:?}", origin, direction);
                      return Some((p, c, min));
                  }
                  return None;
                  */
          }

    pub fn pixel_color(p: &Polygon, c: Vector3<f64>) -> RGB {
        let v : Vec<(RGB, Vector3<f64>)> = [p.0, p.1, p.2].iter().map(
            |x| (x.color, x.coords.into())).collect();

        let colors : Vec<(RGB, f64)> = v.iter().map(
            |&(rgb, x)| (rgb, (x - c).length())).collect();
        let t : f64 = colors.iter().map(|&(_,t)| t).sum();

        let r : f64 = colors.iter().map(|&(c, d)| 1.0-d*c.0 as f64 / t).sum();
        let g : f64 = colors.iter().map(|&(c, d)| 1.0-d*c.1 as f64 / t).sum();
        let b : f64 = colors.iter().map(|&(c, d)| 1.0-d*c.2 as f64 / t).sum();

        RGB(r as u8, g as u8, b as u8)
    }

    pub fn render(&mut self, scene: &World) {
        let origin = Vector3(self.width  as f64 / 2.0,
                             self.height as f64 / 2.0,
                             -1.0);
        let poly = scene.shapes.iter().flat_map(|x| x.to_polygons()).collect();
        let s = Space::new(&poly);
        for a in 0..self.width {
            // println!("{}", a);
            for b in 0..self.height {
                let dir : Vector3<f64> = Vector3(
                    (a as f64 - self.width  as f64 / 2.0),
                    (b as f64 - self.height as f64 / 2.0),
                    1.0);
                let closest = Canvas::closest_polygon(origin.into_inner(),
                                                     dir,
                                                     &s);
                if let Some((p, c, d)) = closest {
                    self.pixels[b as usize][a as usize] =
                        Canvas::pixel_color(&p, c);
                    self.zbuffer[b as usize][a as usize] = d
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

