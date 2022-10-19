use crate::player::PlayEngine;

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

    pub fn speak(&self, engine: &mut impl TtsEgine) {
        engine
            .set_lang(self.lang.clone())
            .set_speed(self.speed.clone())
            .speak(&self.text);
    }

    pub fn play(&self, engine: impl PlayEngine) {
        engine.play(self.path.as_str());
    }

    pub fn stop(&self, engine: impl PlayEngine) {
        engine.stop();
    }
}
