pub mod clipboard;
pub mod selection;
pub mod stdin;

pub trait InputEngine {
    fn input(&self) -> String;
}