pub trait TtsEgine {
    fn speak(&self, text: &str);
}

pub trait PlayEngine {
    fn play(&self, file: &str);
    fn stop(&self);
}

pub struct Tts {
    lang: String,
    speed: String,
}

impl Tts {
    pub fn new() -> Tts {
        Tts {
            lang: String::from("fr-FR"),
            speed: String::from("1"),
        }
    }

    pub fn set_lang(&mut self, lang: String) -> &mut Tts {
        self.lang = lang;
        self
    }

    pub fn set_speed(&mut self, speed: String) -> &mut Tts {
        self.speed = speed;
        self
    }

    pub fn speak(&self, engine: impl TtsEgine, text: &str) {
        engine.speak(text);
    }

    pub fn play(&self, engine: impl PlayEngine) {
        engine.play("/dev/shm/out.wav");
    }

    pub fn stop(&self, engine: impl PlayEngine) {
        engine.stop();
    }
}
