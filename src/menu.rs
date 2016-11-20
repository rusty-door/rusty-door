use tickable::{Tickable,Input};
use state;
use direction;

pub struct MenuScreen<'b> {
    state: &'b mut state::ProgramState,
    subscreen: Subscreens,
}

#[derive(PartialEq, Clone, Copy)]
enum Subscreens {
    Menu(Menu),
    Options(Options),
    HighScore,
    License(u8),
    Quit(Menu, bool),
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

impl<'b> Tickable for MenuScreen<'b> {

    fn tick(&mut self, input: Option<Input>) -> Option<Box<Tickable>> {
        if let Some(i) = input {
            match self.subscreen {
                Subscreens::Menu(m) => {
                    self.tick_menu(i, m)
                },
                Subscreens::Options(o) => {
                    self.tick_opt(i, o);
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
            subscreen: Subscreens::Menu(
                if let Some(_) = state.game {
                    Menu::Continue
                } else {
                    Menu::NewGame
                })
        }
    }

    fn tick_menu(&mut self, input: Input, m: Menu) -> Option<Box<Tickable>> {
        match input {
            Input::Accept => {
                match m {
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
                if d == direction::DIR_DOWN {
                    self.subscreen = Subscreens::Menu(m.next());
                } else if d == direction::DIR_UP {
                    self.subscreen = Subscreens::Menu(m.prev());
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
                    }
                },
                Input::Cancel => {
                    self.subscreen = Subscreens::Menu(Menu::Options);
                },
                _ => ()
            }

        }
}

