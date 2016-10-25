use screen;
use state;

pub struct MenuScreen<'b> {
    state: &'b mut state::ProgramState,
}

impl<'b> screen::Screen for MenuScreen<'b> {
    fn tick<'a>(&'a mut self, input: Option<screen::UserInput>) ->
        &'a mut screen::Screen {
            if let Some(i) = input {
                match i {
                    UserInput::Accept => (),
                    UserInput::Cancel => (),
                    UserInput::Direction(d) => (),
                    UserInput::Menu => (),
                }
            }
            self
        }

    fn new<'a>(state: &'a mut state::ProgramState)
        -> Box<screen::Screen + 'a> {
            Box::new(MenuScreen {
                state: state,
            })
        }
}

