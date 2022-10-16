use crate::tts::tts::TtsEgine;
use std::process::{Command, Stdio};

fn pico_effect(text: &str, speed: i32, pitch: i32, volume: i32) -> String {
    let speed = format!("<speed level=\"{}\">{}</speed>", speed, text);
    let pitch = format!("<pitch level=\"{}\">{}</pitch>", pitch, speed);
    let volume = format!("<volume level=\"{}\">{}</volume>", volume, pitch);
    volume
}

pub struct Pico {}

impl TtsEgine for Pico {
    fn speak(&self, text: &str) {
        let effect = pico_effect(text, 100, 100, 120);

        Command::new("pico2wave")
            .arg("-l=fr-FR")
            .arg("-w=/dev/shm/out.wav")
            .arg(effect)
            .stdout(Stdio::piped())
            .output()
            .expect("failed to execute process");
    }
}
