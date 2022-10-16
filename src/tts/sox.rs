use crate::tts::tts::PlayEngine;
use std::process::{Command, Stdio};

pub struct Sox {}

impl PlayEngine for Sox {
    
    fn play(&self, file: &str) {
        Command::new("sox")       
            .arg(file)
            .stdout(Stdio::piped())
            .output()
            .expect("failed to execute process");
    }

    fn stop(&self) {
        panic!("Not implemented");
    }
}