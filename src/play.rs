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
        let mut shapes : Vec<Shape>   = vec!();
        let mut light  : Vec<Coord3D> = vec!();

        if let Some(ref game) = self.state.game {
            light.push(Coord3D(game.player.x as i32,
                               game.player.y as i32, 1));

            let w = game.field.0.width() as i32;
            let h = game.field.0.height() as i32;

            for x in 0 .. h {
                for y in 0 .. w {
                    let coord_to_vertex = |c: &Coord3D| Vertex {
                                       coords: *c,
                                       color: RGB(0x6A, 0x20, 0x0C)
                    };
                    if game.field.0[Point{x: x as usize, y: y as usize}] {
                        shapes.push(Shape {
                            verts: vec!(
                             Coord3D(( y  ) * 640 / w, (x+1) * 480 / h, 3),
                             Coord3D(( y+1) * 640 / w, (x+1) * 480 / h, 3),
                             Coord3D(( y  ) * 640 / w, (x  ) * 480 / h, 3),
                             Coord3D(( y+1) * 640 / w, (x  ) * 480 / h, 3)).
                                     iter().map(coord_to_vertex).collect(),
                            primitive: Primitive::TriangleStrip,
                        });
                    } else {
                        shapes.push(Shape {
                            verts: vec!(
                             Coord3D(( y  ) * 640 / w, (x+1) * 480 / h, -2),
                             Coord3D(( y+1) * 640 / w, (x+1) * 480 / h, -2),
                             Coord3D(( y  ) * 640 / w, (x  ) * 480 / h, -2),
                             Coord3D(( y+1) * 640 / w, (x  ) * 480 / h, -2)).
                                     iter().map(coord_to_vertex).collect(),
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

