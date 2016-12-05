use tickable::{Tickable,Input};
use state::ProgramState;
use menu::MenuScreen;
use std::mem;

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

impl Tickable for PlayScreen {
    fn tick(&mut self, input: Option<Input>) -> Option<Box<Tickable>> {
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

