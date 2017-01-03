use tickable::Input;
use std::fmt;

pub trait Screen: fmt::Debug {
    fn tick(&mut self, Option<Input>) -> Option<Box<Screen>>;
}
