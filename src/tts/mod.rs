pub mod espeak;
pub mod pico;

use crate::{player::PlayEngine, utils::command_exists};

pub trait TtsEgine {
    fn new() -> Self;
    fn speak(&self, text: &str);
    fn set_lang(&mut self, lang: String) -> &mut Self;
    fn set_speed(&mut self, speed: i32) -> &mut Self;
}

pub struct Tts {
    pub lang: String,
    pub speed: i32,
    pub path: String,
    pub text: String,
}

impl Tts {
    pub fn new() -> Tts {
        Tts {
            lang: String::from("fr-FR"),
            speed: 1,
            path: String::from("/dev/shm/out.wav"),
            text: String::new(),
        }
    }

    pub fn set_lang(&mut self, lang: String) -> &mut Tts {
        self.lang = lang;
        self
    }

    pub fn set_speed(&mut self, speed: i32) -> &mut Tts {
        self.speed = speed;
        self
    }

    pub fn set_text(&mut self, text: String) -> &mut Tts {
        self.text = text;
        self
    }

    pub fn speak(&mut self, engine: &mut impl TtsEgine) -> &mut Tts {
        engine
            .set_lang(self.lang.clone())
            .set_speed(self.speed)
            .speak(&self.text);

        self
    }

    pub fn play(&self, engine: impl PlayEngine) {
        engine.play(self.path.as_str());
    }

    pub fn stop(&self, engine: impl PlayEngine) {
        engine.stop();
    }

    pub fn list_available_engines() -> Vec<&'static str> {
        let engines = ["pico", "espeak"];

        engines
            .iter()
            .filter(|&&engine| match engine {
                "pico" => command_exists("pico2wave"),
                "espeak" => command_exists("espeak"),
                _ => false,
            })
            .cloned()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::{espeak::Espeak, *};

    #[test]
    fn test_tts() {
        let _ = std::fs::remove_file("/dev/shm/out.wav");

        let mut tts = Tts::new();
        tts.set_lang(String::from("fr-FR"))
            .set_speed(1)
            .set_text(String::from("Bonjour"))
            .speak(&mut Espeak::new());

        assert!(std::path::Path::new("/dev/shm/out.wav").exists());
    }
}
