use crate::input::InputEngine;
use std::process::Command;

pub struct Selection {}

impl InputEngine for Selection {
    fn input(&self) -> String {
        let output = Command::new("xclip")
            .arg("-o")
            .output()
            .expect("failed to execute process");

        let clipboard = String::from_utf8_lossy(&output.stdout).to_string();

        clipboard
    }
}
