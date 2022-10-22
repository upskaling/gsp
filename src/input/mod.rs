pub mod clipboard;
pub mod selection;
pub mod stdin;
pub mod ocr;

pub trait InputEngine {
    fn input(&self) -> String;
}