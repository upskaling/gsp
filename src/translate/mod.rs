mod argos_translate;
mod libretranslate;
mod translate_locally;

use lingua::Language::{English, French};
use lingua::LanguageDetectorBuilder;

pub trait TranslateEngine {
    fn translate(&self, text: &str, lang_from: &str, lang_to: &str) -> String;
    fn is_available(&self) -> bool;
}

pub struct Translate {}

impl Translate {
    pub fn new() -> Translate {
        Translate {}
    }

    pub fn translate(&self, engine: &str, text: &str, lang_from: &str, lang_to: &str) -> String {
        let mut lang_from = lang_from.to_string();

        if lang_from == "auto" {
            let languages = vec![English, French];
            let detector = LanguageDetectorBuilder::from_languages(&languages).build();

            lang_from = match detector.detect_language_of(text).unwrap_or(French) {
                English => "en-US".to_string(),
                French => "fr-FR".to_string(),
            };

            if lang_from == lang_to {
                return text.to_string();
            }
        }

        match engine {
            "translate_locally" => {
                translate_locally::TranslateLocally {}.translate(text, &lang_from, lang_to)
            }
            "argos_translate" => {
                argos_translate::ArgosTranslate {}.translate(text, &lang_from, lang_to)
            }
            "libretranslate" => {
                libretranslate::Libretranslate {}.translate(text, &lang_from, lang_to)
            }
            _ => libretranslate::Libretranslate {}.translate(text, &lang_from, lang_to),
        }
    }

    pub fn list_available_engines() -> Vec<&'static str> {
        let engines = ["libretranslate", "argos_translate", "translate_locally"];

        engines
            .iter()
            .filter(|&&engine| match engine {
                "libretranslate" => libretranslate::Libretranslate {}.is_available(),
                "argos_translate" => argos_translate::ArgosTranslate {}.is_available(),
                "translate_locally" => translate_locally::TranslateLocally {}.is_available(),
                _ => false,
            })
            .cloned()
            .collect()
    }
}
