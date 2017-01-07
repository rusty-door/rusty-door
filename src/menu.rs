use tickable::Input;
use screen::Screen;
use state::ProgramState;
use direction;
use license;
use play::PlayScreen;
use geometry;
use std::mem;

#[derive(Debug)]
pub struct MenuScreen {
    state: ProgramState,
    subscreen: Subscreens,
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum Subscreens {
    Menu(Menu),
    Options(Options),
    HighScore,
    License(u16),
    Quit(Menu, bool),
}

#[derive(PartialEq, Clone, Copy, Debug)]
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
        OPTION_ITEMS[(self.position().unwrap_or(0) + 1)
            % OPTION_ITEMS.len()]
    }

    fn prev(&self) -> Options {
        OPTION_ITEMS[(self.position().unwrap_or(0) + OPTION_ITEMS.len() - 1)
            % OPTION_ITEMS.len()]
    }

    fn position(&self) -> Option<usize> {
        OPTION_ITEMS.iter().position(|x| self.eq(x))
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum Menu {
    NewGame,
    Continue,
    Options,
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
        MENU_ITEMS[(self.position().unwrap_or(0) + 1)
            % MENU_ITEMS.len()]
    }

    fn prev(&self) -> Menu {
        MENU_ITEMS[(self.position().unwrap_or(0) + MENU_ITEMS.len() - 1)
            % MENU_ITEMS.len()]
    }

    fn position(&self) -> Option<usize> {
        MENU_ITEMS.iter().position(|x| self.eq(x))
    }
}

impl geometry::Worldly for MenuScreen {
    fn scene(&self) -> geometry::World {
        geometry::World {
            polygons : vec!(),
            lighting : vec!()
        }
    }
}

impl Screen for MenuScreen {

    fn tick(&mut self, input: Option<Input>) -> Option<Box<Screen>> {
        if let Some(i) = input {
            match self.subscreen {
                Subscreens::Menu(m) => {
                    self.tick_menu(i, m)
                },
                Subscreens::Options(o) => {
                    self.tick_opt(i, o);
                    None
                },
                Subscreens::License(line) => {
                    self.tick_license(i, line);
                    None
                },
                Subscreens::HighScore => {
                    self.tick_highscore(i);
                    None
                },
                Subscreens::Quit(m, q) => {
                    self.tick_quit(i, m, q);
                    None
                }
            }
        } else {
            None
        }
    }

}

impl MenuScreen {
    pub fn new(state: ProgramState) -> MenuScreen {
        MenuScreen {
            subscreen: Subscreens::Menu(
                if state.game.is_some() {
                    Menu::Continue
                } else {
                    Menu::NewGame
                }),
            state: state,
        }
    }

    fn tick_menu(&mut self, input: Input, m: Menu) -> Option<Box<Screen>> {
        match input {
            Input::Accept => {
                match m {
                    Menu::NewGame => {
                        self.state.new_game();
                        Some(Box::new(PlayScreen::new(mem::replace(
                            &mut self.state,
                            ProgramState::new()))))
                    },
                    Menu::Continue => {
                        Some(Box::new(PlayScreen::new(mem::replace(
                            &mut self.state,
                            ProgramState::new()))))
                    },
                    Menu::Options => {
                        self.subscreen = Subscreens::Options(
                            Options::Width);
                        None
                    },
                    Menu::HighScore => {
                        self.subscreen = Subscreens::HighScore;
                        None
                    },
                    Menu::License => {
                        self.subscreen = Subscreens::License(0);
                        None
                    },
                    Menu::Quit => {
                        self.subscreen = Subscreens::Quit(m, true);
                        None
                    },
                }
            },
            Input::Direction(d) => {
                let it = |m : Menu| {
                    if d == direction::DIR_DOWN {
                        m.next()
                    } else if d == direction::DIR_UP {
                        m.prev()
                    } else {
                        m
                    }
                };
                self.subscreen = Subscreens::Menu(it(m));
                if let Subscreens::Menu(Menu::Continue) = self.subscreen {
                    if let None = self.state.game {
                        self.subscreen = Subscreens::Menu(it(Menu::Continue));
                    }
                }
                None
            },
            Input::Cancel => {
                self.subscreen = Subscreens::Quit(m, false);
                None
            },
            _ => None
        }
    }

    fn tick_opt(&mut self, input: Input, o: Options) {
            match input {
                Input::Direction(d) => {
                    if d == direction::DIR_DOWN {
                        self.subscreen = Subscreens::Options(o.next());
                    } else if d == direction::DIR_UP {
                        self.subscreen = Subscreens::Options(o.prev());
                    } else if d == direction::DIR_LEFT {
                        match o {
                            Options::Width =>
                                self.state.width  = self.state.width  - 1,
                            Options::Height =>
                                self.state.height = self.state.height - 1,
                            Options::Seed => {
                                self.state.seed =
                                    self.state.seed.map(|x| x - 1)
                            }
                        }
                    } else if d == direction::DIR_RIGHT {
                        match o {
                            Options::Width =>
                                self.state.width  = self.state.width  + 1,
                            Options::Height =>
                                self.state.height = self.state.height + 1,
                            Options::Seed => {
                                self.state.seed =
                                    self.state.seed.map(|x| x + 1)
                            }
                        }
                    }
                },
                Input::Cancel => {
                    self.subscreen = Subscreens::Menu(Menu::Options);
                },
                _ => ()
            }
        }

    fn tick_license(&mut self, input: Input, line: u16) {
        match input {
            Input::Direction(d) => {
                if d == direction::DIR_DOWN {
                    self.subscreen = Subscreens::License(line + 1);
                } else if d == direction::DIR_UP {
                    self.subscreen = Subscreens::License(
                        if line > 0 {
                            line - 1
                        } else {
                            line
                        });
                }
            },
            Input::Cancel => {
                self.subscreen = Subscreens::Menu(Menu::License);
            },
            _ => ()
        }
    }

    fn tick_quit(&mut self, input: Input, menu: Menu, quit: bool) {
        match input {
            Input::Direction(_) => {
                self.subscreen = Subscreens::Quit(menu, !quit);
            },
            Input::Cancel => {
                self.subscreen = Subscreens::Menu(menu);
            },
            Input::Accept => {
                if quit {
                    self.subscreen = Subscreens::Menu(menu);
                    // TODO: Actually quit.
                } else {
                    self.subscreen = Subscreens::Menu(menu);
                }
            },
            _ => ()
        }
    }

    fn tick_highscore(&mut self, input: Input) {
        match input {
            Input::Cancel => {
                self.subscreen = Subscreens::Menu(Menu::HighScore);
            },
            _ => ()
        }
    }
}

