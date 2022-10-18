use std::process::{Command, Stdio};

use crate::tts::tts::TtsEgine;

pub struct Espeak {
    pub lang: String,
    pub speed: i32,
    pub pitch: i32,
    pub volume: i32,
    pub path: String,
}

impl TtsEgine for Espeak {
    fn speak(&self, text: &str) {
        let speed = (self.speed / 100 * 320) / 2;

        Command::new("espeak")
            .arg("-v")
            .arg(format!("mb-{}{}", self.lang[..2].to_uppercase(), "4"))
            .arg("-s")
            .arg(speed.to_string())
            .arg("-p")
            .arg("45")
            .arg("-a")
            .arg("100")
            .arg("-w")
            .arg(self.path.as_str())
            .arg(text)
            .stdout(Stdio::piped())
            .output()
            .expect("failed to execute process");
    }

    fn new() -> Self {
        Espeak {
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
