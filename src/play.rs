use tickable::Input;
use tickable::Tickable;
use screen::Screen;
use state::ProgramState;
use menu::MenuScreen;
use geometry::*;
use std::mem;
use labyrinth::Point;

#[derive(Debug)]
pub struct PlayScreen {
    state: ProgramState,
}

impl PlayScreen {
    pub fn new(state: ProgramState) -> PlayScreen {
        PlayScreen {
            state: state,
        }
    }
}

impl Worldly for PlayScreen {
    fn scene(&self) -> World {
        let mut shapes : Vec<Shape>        = vec!();
        let mut light  : Vec<Vector3<f64>> = vec!();

        if let Some(ref game) = self.state.game {
            light.push(Vector3(game.player.x as i32, game.player.y as i32,
                              1).into_inner());

            let zf = 0.3;
            let zn = 0.26;

            let w = game.field.0.width() as i32;
            let h = game.field.0.height() as i32;

            for x in 0 .. h {
                for y in 0 .. w {
                    let coord_to_vertex = |r: RGB, c: &Vector3<f64>| Vertex {
                                       coords: *c,
                                       color: r
                    };
                    if game.field.0[Point{x: x as usize, y: y as usize}] {
                        let r = RGB(0x61, 0x40, 0x20);
                        shapes.push(Shape {
                            verts: vec!(
                             Vector3((( y  ) * 640 / w) as f64, ((x+1) * 480 / h) as f64, zn),
                             Vector3((( y+1) * 640 / w) as f64, ((x+1) * 480 / h) as f64, zn),
                             Vector3((( y  ) * 640 / w) as f64, ((x  ) * 480 / h) as f64, zn),
                             Vector3((( y+1) * 640 / w) as f64, ((x  ) * 480 / h) as f64, zn)).
                                     iter().map(|c| coord_to_vertex(r, c)).
                                     collect(),
                            primitive: Primitive::TriangleStrip,
                        });

                        let m = RGB(0x33, 0x41, 0x33);
                        if y > 0 && !*game.field.0.get(Point{x:x as usize, y: (y-1) as usize}).unwrap_or(&false) {
                            shapes.push(Shape {
                                verts: vec!(
                                 Vector3((( y  ) * 640 / w) as f64, ((x+1) * 480 / h) as f64, zn),
                                 Vector3((( y  ) * 640 / w) as f64, ((x  ) * 480 / h) as f64, zn),
                                 Vector3((( y  ) * 640 / w) as f64, ((x+1) * 480 / h) as f64, zf),
                                 Vector3((( y  ) * 640 / w) as f64, ((x  ) * 480 / h) as f64, zf)).
                                         iter().map(|c| coord_to_vertex(m, c)).
                                         collect(),
                                primitive: Primitive::TriangleStrip,
                            });
                        }
                        let n = RGB(0x41, 0x33, 0x41);
                        if !*game.field.0.get(Point{x:x as usize, y: (y+1) as usize}).unwrap_or(&false) {
                            shapes.push(Shape {
                                verts: vec!(
                                 Vector3((( y+1 ) * 640 / w) as f64, ((x+1) * 480 / h) as f64, zn),
                                 Vector3((( y+1 ) * 640 / w) as f64, ((x  ) * 480 / h) as f64, zn),
                                 Vector3((( y+1 ) * 640 / w) as f64, ((x+1) * 480 / h) as f64, zf),
                                 Vector3((( y+1 ) * 640 / w) as f64, ((x  ) * 480 / h) as f64, zf)).
                                         iter().map(|c| coord_to_vertex(n, c)).
                                         collect(),
                                primitive: Primitive::TriangleStrip,
                            });
                        }
                        let o = RGB(0x71, 0x15, 0xAA);
                        if x > 0 && !*game.field.0.get(Point{x:(x-1) as usize, y: y as usize}).unwrap_or(&false) {
                            shapes.push(Shape {
                                verts: vec!(
                                 Vector3((( y+1 ) * 640 / w) as f64, ((x  ) * 480 / h) as f64, zn),
                                 Vector3((( y   ) * 640 / w) as f64, ((x  ) * 480 / h) as f64, zn),
                                 Vector3((( y+1 ) * 640 / w) as f64, ((x  ) * 480 / h) as f64, zf),
                                 Vector3((( y   ) * 640 / w) as f64, ((x  ) * 480 / h) as f64, zf)).
                                         iter().map(|c| coord_to_vertex(o, c)).
                                         collect(),
                                primitive: Primitive::TriangleStrip,
                            });
                        }
                        let p = RGB(0x33, 0xBB, 0x33);
                        if !*game.field.0.get(Point{x:(x+1) as usize, y: y as usize}).unwrap_or(&false) {
                            shapes.push(Shape {
                                verts: vec!(
                                 Vector3((( y+1 ) * 640 / w) as f64, ((x+1) * 480 / h) as f64, zn),
                                 Vector3((( y   ) * 640 / w) as f64, ((x+1) * 480 / h) as f64, zn),
                                 Vector3((( y+1 ) * 640 / w) as f64, ((x+1) * 480 / h) as f64, zf),
                                 Vector3((( y   ) * 640 / w) as f64, ((x+1) * 480 / h) as f64, zf)).
                                         iter().map(|c| coord_to_vertex(p, c)).
                                         collect(),
                                primitive: Primitive::TriangleStrip,
                            });
                        }
                    } else {
                        let r = RGB(0x40, 0x20, 0x61);
                        shapes.push(Shape {
                            verts: vec!(
                             Vector3((( y  ) * 640 / w) as f64, ((x+1) * 480 / h) as f64, zf),
                             Vector3((( y+1) * 640 / w) as f64, ((x+1) * 480 / h) as f64, zf),
                             Vector3((( y  ) * 640 / w) as f64, ((x  ) * 480 / h) as f64, zf),
                             Vector3((( y+1) * 640 / w) as f64, ((x  ) * 480 / h) as f64, zf)).
                                     iter().map(|c| coord_to_vertex(r, c)).
                                     collect(),
                            primitive: Primitive::TriangleStrip,
                        });
                    }

                    let player = Shape {
                        verts: vec!(
                            Vertex {
                                color: RGB(0x00, 0xFF, 0xFF),
                                coords: Vector3((game.player.y as i32 * 640 / w) as f64, (game.player.x as i32 * 480 / h) as f64, (zf + zn) / 2.0)
                            },
                            Vertex {
                                color: RGB(0xFF, 0x00, 0xFF),
                                coords: Vector3((game.player.y as i32 * 640 / w) as f64, ((game.player.x as i32 + 1) * 480 / h) as f64, (zf + zn) / 2.0)
                            },
                            Vertex {
                                color: RGB(0xFF, 0xFF, 0x00),
                                coords: Vector3(((game.player.y as i32 + 1) * 640 / w) as f64, (game.player.x as i32 * 480 / h) as f64, (zf + zn) / 2.0)
                            }),
                        primitive: Primitive::TriangleStrip
                    };

                    shapes.push(player);
                }
            }
        }

        World {
            shapes   : shapes,
            lighting : light
        }
    }
}

impl Screen for PlayScreen {
    fn tick(&mut self, input: Option<Input>) -> Option<Box<Screen>> {
        match input {
            Some(Input::Menu) =>
                Some(Box::new(MenuScreen::new(mem::replace(
                                &mut self.state,
                                ProgramState::new())))),
            _ => {
                let mut finished = false;
                if let Some(ref mut game) = self.state.game {
                    game.tick(input);
                    finished = game.is_finished();
                }
                if finished {
                    self.state.game = None;
                    Some(Box::new(MenuScreen::new(mem::replace(
                                    &mut self.state,
                                    ProgramState::new()))))
                } else {
                    None
                }
            }
        }
    }
}

