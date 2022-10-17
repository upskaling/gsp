use crate::player::PlayEngine;
use psutil::process::processes;
use std::process::{Command, Stdio};

pub struct Paplay {}

impl PlayEngine for Paplay {
    fn play(&self, file: &str) {
        Command::new("paplay")
            .arg("--client-name=gsp")
            .arg(file)
            .stdout(Stdio::piped())
            .output()
            .expect("failed to execute process");
    }

    fn stop(&self) {
        for process in processes().unwrap() {
            let process = process.unwrap();

            if process.name().unwrap() != "paplay" {
                continue;
            }

            let cmdline = process.cmdline().unwrap().unwrap();

            if cmdline.contains(&"--client-name=gsp".to_string()) {
                process.kill().unwrap();
                println!("killed paplay process {}", process.pid());
            }
        }
    }
}
