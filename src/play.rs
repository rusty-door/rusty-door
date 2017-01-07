use tickable::Input;
use tickable::Tickable;
use screen::Screen;
use state::ProgramState;
use menu::MenuScreen;
use geometry;
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

impl geometry::Worldly for PlayScreen {
    fn scene(&self) -> geometry::World {
        let mut shapes  : Vec<geometry::Shape>   = vec!();
        let mut light : Vec<geometry::Coord3D> = vec!();

        if let Some(ref game) = self.state.game {
            light.push(geometry::Coord3D(game.player.x as i32,
                                         game.player.y as i32, 1));

            for x in 0 .. game.field.0.height() {
                for y in 0 .. game.field.0.width() {
                    if game.field.0[Point{x: x, y: y}] {
                        shapes.push(geometry::Shape {
                            coords: vec!(
                             geometry::Coord3D((x  ) as i32,( y  ) as i32, 2),
                             geometry::Coord3D((x+1) as i32,( y  ) as i32, 2),
                             geometry::Coord3D((x+1) as i32,( y+1) as i32, 2),
                             geometry::Coord3D((x  ) as i32,( y+1) as i32, 2)),
                            primitive: geometry::Primitive::TriangleStrip,
                            color: geometry::ShapeColor::Const(
                             geometry::RGB(0x6A, 0x20, 0x15))
                        });
                    } else {
                        shapes.push(geometry::Shape {
                            coords: vec!(
                             geometry::Coord3D((x  ) as i32,( y  ) as i32, 0),
                             geometry::Coord3D((x+1) as i32,( y  ) as i32, 0),
                             geometry::Coord3D((x+1) as i32,( y+1) as i32, 0),
                             geometry::Coord3D((x  ) as i32,( y+1) as i32, 0)),
                            primitive: geometry::Primitive::TriangleStrip,
                            color: geometry::ShapeColor::Const(
                             geometry::RGB(0x6A, 0x20, 0x15))
                        });
                    }
                }
            }
        }

        geometry::World {
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

