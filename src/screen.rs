use tickable::Input;
use geometry::Worldly;
use std::fmt;

pub trait Screen: fmt::Debug + Worldly {
    fn tick(&mut self, Option<Input>) -> Option<Box<Screen>>;
}
