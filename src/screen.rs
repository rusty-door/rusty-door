use tickable::Input;

pub trait Screen {
    fn tick(&mut self, Option<Input>) -> Option<Box<Screen>>;
}
