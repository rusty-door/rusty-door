use screen;
use state;

pub struct MenuScreen<'b> {
    state: &'b mut state::ProgramState,
}

impl<'b> screen::Screen for MenuScreen<'b> {
    fn tick(&mut self, input: Option<screen::UserInput>) ->
        Option<Box<screen::Screen>> {
            if let Some(i) = input {
                match i {
                    screen::UserInput::Accept => self.state.new_game(),
                    screen::UserInput::Cancel => (),
                    screen::UserInput::Direction(_) => (),
                    screen::UserInput::Menu =>
                      if let Some(ref f) = self.state.game {
                          println!("{}", f.field);
                      },
                }
            }
            None
        }

}

impl<'b> MenuScreen<'b> {
    pub fn new<'a>(state: &'a mut state::ProgramState) -> MenuScreen<'a> {
        MenuScreen {
            state: state,
        }
    }
}

