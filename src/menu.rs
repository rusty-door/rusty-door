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
    Seed,
}

const OPTION_ITEMS: [Options; 3] = [
    Options::Width,
    Options::Height,
    Options::Seed,
];

impl Options {
    fn next(&self) -> Options {
        OPTION_ITEMS[OPTION_ITEMS.iter().position(
            |x| self.eq(x)).unwrap_or(0)]
    }

    fn prev(&self) -> Options {
        OPTION_ITEMS[OPTION_ITEMS.iter().rposition(
            |x| self.eq(x)).unwrap_or(0)]
    }
}

#[derive(PartialEq, Clone, Copy)]
enum Menu {
    NewGame,
    Continue,
    Options,
    OptionsIn(Options),
    HighScore,
    HighScoreIn,
    License,
    LicenseIn(u16),
    Quit,
    QuitIn(usize, bool),
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
        MENU_ITEMS[self.position().unwrap_or(0) + 1
            % MENU_ITEMS.len()]
    }

    fn prev(&self) -> Menu {
        MENU_ITEMS[self.position().unwrap_or(0) + MENU_ITEMS.len() - 1
            % MENU_ITEMS.len()]
    }

    fn position(&self) -> Option<usize> {
        MENU_ITEMS.iter().position(|x| self.eq(x))
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
                                None // TODO: start game
                            },
                            Menu::Continue => {
                                if let Some(ref f) = self.state.game {
                                    println!("{}", f.field);
                                }
                                None // TODO: start game
                            },
                            Menu::Options => {
                                self.menu = Menu::OptionsIn(Options::Width);
                                None
                            },
                            Menu::HighScore => {
                                self.menu = Menu::HighScoreIn;
                                None
                            },
                            Menu::License => {
                                self.menu = Menu::LicenseIn(0);
                                None
                            },
                            Menu::Quit => {
                                self.menu = Menu::QuitIn(
                                    self.menu.position().unwrap_or(0), true);
                                None
                            },
                            _ => None
                        }
                    },
                    screen::UserInput::Direction(d) => {
                        if let Menu::OptionsIn(o) = self.menu {
                            if d == direction::DIR_DOWN {
                                self.menu = Menu::OptionsIn(o.next());
                            } else if d == direction::DIR_UP {
                                self.menu = Menu::OptionsIn(o.prev());
                            } // TODO: change settings
                        // TODO: navigate license and quitting dialogue
                        } else {
                            if d == direction::DIR_DOWN {
                                self.menu = self.menu.next();
                            } else if d == direction::DIR_UP {
                                self.menu = self.menu.prev();
                            }
                        }
                        None
                    },
                    screen::UserInput::Cancel => {
                        if let Menu::OptionsIn(_) = self.menu {
                            self.menu = Menu::Options;
                        } else if let Menu::QuitIn(m, _) = self.menu {
                            self.menu = MENU_ITEMS[m];
                        } else if let Menu::LicenseIn(_) = self.menu {
                            self.menu = Menu::License;
                        } else if let Menu::HighScoreIn = self.menu {
                            self.menu = Menu::HighScore;
                        } else {
                            self.menu = Menu::QuitIn(
                                self.menu.position().unwrap_or(0), false);
                        }
                        None
                    },
                    _ => None
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

