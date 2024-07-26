use std::process::Command;

use super::TtsEgine;

pub struct Espeak {
    pub lang: String,
    pub speed: i32,
    pub path: String,
}

impl TtsEgine for Espeak {
    fn speak(&self, text: &str) {
        let speed = (self.speed / 100 * 320) / 2;

        Command::new("espeak")
            // voice name
            .arg("-v")
            .arg(format!("mb-{}{}", self.lang[..2].to_uppercase(), "4"))
            // speed
            .arg("-s")
            .arg(speed.to_string())
            // pitch
            .arg("-p")
            .arg("45")
            // volume
            .arg("-a")
            .arg("100")
            // path
            .arg("-w")
            .arg(self.path.as_str())
            .arg("--")
            // text
            .arg(text)
            .output()
            .expect("failed to execute process");
    }

    fn new() -> Self {
        Espeak {
            lang: String::from("fr-FR"),
            speed: 1,
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
