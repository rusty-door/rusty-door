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

            let w = game.field.0.width() as i32;
            let h = game.field.0.height() as i32;

            for x in 0 .. h {
                for y in 0 .. w {
                    let coord_to_vertex = |r: RGB, c: &Vector3<i32>| Vertex {
                                       coords: c.into_inner(),
                                       color: r
                    };
                    if game.field.0[Point{x: x as usize, y: y as usize}] {
                        let r = RGB(0x61, 0x40, 0x20);
                        shapes.push(Shape {
                            verts: vec!(
                             Vector3(( y  ) * 640 / w, (x+1) * 480 / h, 3),
                             Vector3(( y+1) * 640 / w, (x+1) * 480 / h, 3),
                             Vector3(( y  ) * 640 / w, (x  ) * 480 / h, 3),
                             Vector3(( y+1) * 640 / w, (x  ) * 480 / h, 3)).
                                     iter().map(|c| coord_to_vertex(r, c)).
                                     collect(),
                            primitive: Primitive::TriangleStrip,
                        });
                    } else {
                        let r = RGB(0x40, 0x20, 0x61);
                        shapes.push(Shape {
                            verts: vec!(
                             Vector3(( y  ) * 640 / w, (x+1) * 480 / h, 4),
                             Vector3(( y+1) * 640 / w, (x+1) * 480 / h, 4),
                             Vector3(( y  ) * 640 / w, (x  ) * 480 / h, 4),
                             Vector3(( y+1) * 640 / w, (x  ) * 480 / h, 4)).
                                     iter().map(|c| coord_to_vertex(r, c)).
                                     collect(),
                            primitive: Primitive::TriangleStrip,
                        });
                    }
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

