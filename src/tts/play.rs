use crate::tts::tts::PlayEngine;
use psutil::process::processes;
use std::process::{Command, Stdio};

pub struct Play {}

impl PlayEngine for Play {
    fn play(&self, file: &str) {
        Command::new("play")
            .arg(file)
            .stdout(Stdio::piped())
            .output()
            .expect("failed to execute process");
    }

    fn stop(&self) {
        for process in processes().unwrap() {
            let process = process.unwrap();

            if process.name().unwrap() != "play" {
                continue;
            }

            process.kill().unwrap();
            println!("killed play process {}", process.pid());
        }
    }
}
