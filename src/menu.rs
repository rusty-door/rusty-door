use screen;
use state;
use direction;

pub struct MenuScreen<'b> {
    state: &'b mut state::ProgramState,
    menu: Menu,
}

#[derive(PartialEq, Clone, Copy)]
enum Options {
    Width,
    Height,
}

#[derive(PartialEq, Clone, Copy)]
enum Menu {
    NewGame,
    Continue,
    Options,
    OptionsIn(Options),
    HighScore,
    License,
    Quit,
}

const MENU_ITEMS: [Menu; 6] = [
    Menu::NewGame,
    Menu::Continue,
    Menu::Options,
    Menu::HighScore,
    Menu::License,
    Menu::Quit,
];

impl Menu {
    fn next(&self) -> Menu {
        MENU_ITEMS[MENU_ITEMS.iter().position(|x| self.eq(x)).unwrap_or(0)]
    }

    fn prev(&self) -> Menu {
        MENU_ITEMS[MENU_ITEMS.iter().rposition(|x| self.eq(x)).unwrap_or(0)]
    }
}

impl<'b> screen::Screen for MenuScreen<'b> {
    fn tick(&mut self, input: Option<screen::UserInput>) ->
        Option<Box<screen::Screen>> {
            if let Some(i) = input {
                match i {
                    screen::UserInput::Accept => {
                        match self.menu {
                            Menu::NewGame => {
                                self.state.new_game();
                                if let Some(ref f) = self.state.game {
                                    println!("{}", f.field);
                                }
                                None
                            },
                            Menu::Options => {
                                self.menu = Menu::OptionsIn(Options::Width);
                                None
                            },
                            _ => None
                        }
                    },
                    screen::UserInput::Direction(d) => {
                        if let Menu::OptionsIn(_) = self.menu {
                        } else {
                            if d == direction::DIR_DOWN {
                                self.menu = self.menu.next();
                            } else if d == direction::DIR_UP {
                                self.menu = self.menu.prev();
                            }
                        }
                        None
                    },
                    _ => None,
                }
            } else {
                None
            }
        }

}

impl<'b> MenuScreen<'b> {
    pub fn new<'a>(state: &'a mut state::ProgramState) -> MenuScreen<'a> {
        MenuScreen {
            state: state,
            menu: if let Some(_) = state.game {
                Menu::Continue
            } else {
                Menu::NewGame
            }
        }
    }
}

