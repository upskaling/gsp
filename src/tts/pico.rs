use crate::tts::tts::TtsEgine;
use std::process::{Command, Stdio};

fn pico_effect(text: &str, speed: i32, pitch: i32, volume: i32) -> String {
    let speed = format!("<speed level=\"{}\">{}</speed>", speed, text);
    let pitch = format!("<pitch level=\"{}\">{}</pitch>", pitch, speed);
    let volume = format!("<volume level=\"{}\">{}</volume>", volume, pitch);
    volume
}

pub struct Pico {
    pub lang: String,
    pub speed: i32,
    pub pitch: i32,
    pub volume: i32,
    pub path: String,
}

impl TtsEgine for Pico {
    fn speak(&self, text: &str) {
        let effect = pico_effect(text, self.speed, 100, 120);

        Command::new("pico2wave")
            .arg(format!("--lang={}", self.lang))
            .arg(format!("-w={}", self.path))
            .arg(effect)
            .stdout(Stdio::piped())
            .output()
            .expect("failed to execute process");
    }

    fn new() -> Self {
        Pico {
            lang: String::from("fr-FR"),
            speed: 1,
            pitch: 1,
            volume: 1,
            path: String::from("/dev/shm/out.wav"),
        }
    }

    fn set_lang(&mut self, lang: String) -> &mut Self {
        self.lang = lang;
        self
    }

    fn set_speed(&mut self, speed: i32) -> &mut Self {
        self.speed = speed;
        self
    }
}
