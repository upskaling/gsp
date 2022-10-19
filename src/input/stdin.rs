use crate::input::InputEngine;

pub struct Stdin {}

impl InputEngine for Stdin {
    fn input(&self) -> String {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        input
    }
}
