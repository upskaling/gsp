use std::process::Command;

use super::TtsEgine;

pub struct EspeakNg {
    lang: String,
    speed: i32,
    pitch: i32,
    amplitude: i32,
    output_file: String,
}

impl Default for EspeakNg {
    fn default() -> Self {
        Self {
            lang: "fr-FR".to_string(),
            speed: 1,
            pitch: 45,
            amplitude: 100,
            output_file: "/dev/shm/out.wav".to_string(),
        }
    }
}

impl EspeakNg {
    pub fn new() -> Self {
        Self::default()
    }
}

impl TtsEgine for EspeakNg {
    fn speak(&self, text: &str) {
        let speed = (self.speed / 100 * 320) / 2;

        Command::new("espeak-ng")
            // voice name
            .arg("-v")
            .arg(format!("mb-{}{}", self.lang[..2].to_uppercase(), "4"))
            // speed
            .arg("-s")
            .arg(speed.to_string())
            // pitch
            .arg("-p")
            .arg(self.pitch.to_string())
            // volume
            .arg("-a")
            .arg(self.amplitude.to_string())
            // path
            .arg("-w")
            .arg(self.output_file.as_str())
            .arg("--")
            // text
            .arg(text)
            .output()
            .expect("failed to execute process");
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
