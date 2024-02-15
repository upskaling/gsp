mod argos_translate;
mod libretranslate;
mod translate_locally;

pub trait TranslateEngine {
    fn translate(&self, text: &str, lang_from: &str, lang_to: &str) -> String;
}

pub struct Translate {}

impl Translate {
    pub fn new() -> Translate {
        Translate {}
    }

    pub fn translate(&self, engine: &str, text: &str, lang_from: &str, lang_to: &str) -> String {
        match engine {
            "translate_locally" => {
                translate_locally::TranslateLocally {}.translate(text, lang_from, lang_to)
            }
            "argos_translate" => {
                argos_translate::ArgosTranslate {}.translate(text, lang_from, lang_to)
            }
            "libretranslate" => {
                libretranslate::Libretranslate {}.translate(text, lang_from, lang_to)
            }
            _ => libretranslate::Libretranslate {}.translate(text, lang_from, lang_to),
        }
    }
}
