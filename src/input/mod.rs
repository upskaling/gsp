pub mod clipboard;
pub mod selection;

pub trait InputEngine {
    fn input(&self) -> String;
}