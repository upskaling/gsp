use super::InputEngine;

pub struct Stdin {}

impl InputEngine for Stdin {
    fn input(&self) -> String {
        let mut input = String::new();
        let mut line = String::new();
        loop {
            std::io::stdin().read_line(&mut line).unwrap();
            input.push_str(&line);
            if line.is_empty() {
                break;
            }
            line.clear();
        }

        input
    }
}
