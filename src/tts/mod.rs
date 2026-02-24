//! Module de synthèse vocale (Text-To-Speech)
//!
//! Fournit une interface unifiée pour les différents moteurs TTS.

use crate::{player::PlayEngine, utils::command_exists};

pub mod espeak;
pub mod espeakng;
pub mod pico;

/// Trait pour les moteurs de synthèse vocale
pub trait TtsEngine {
    /// Génère la parole à partir du texte
    fn speak(&self, text: &str);
    /// Définit la langue du moteur
    fn set_lang(&mut self, lang: String) -> &mut Self;
    /// Définit la vitesse de lecture
    fn set_speed(&mut self, speed: i32) -> &mut Self;
}

/// Configuration principale pour la synthèse vocale
#[derive(Debug, Clone)]
pub struct Tts {
    /// Langue cible (ex: "fr-FR", "en-US")
    pub lang: String,
    /// Vitesse de lecture (pourcentage)
    pub speed: i32,
    /// Chemin du fichier audio de sortie
    pub path: String,
    /// Texte à synthétiser
    pub text: String,
}

impl Default for Tts {
    fn default() -> Self {
        Self::new()
    }
}

impl Tts {
    /// Crée une nouvelle configuration TTS avec des valeurs par défaut
    pub fn new() -> Tts {
        Tts {
            lang: String::from("fr-FR"),
            speed: 1,
            path: String::from("/dev/shm/out.wav"),
            text: String::new(),
        }
    }

    /// Définit la langue cible
    pub fn set_lang(&mut self, lang: String) -> &mut Tts {
        self.lang = lang;
        self
    }

    /// Définit la vitesse de lecture
    pub fn set_speed(&mut self, speed: i32) -> &mut Tts {
        self.speed = speed;
        self
    }

    /// Définit le texte à synthétiser
    pub fn set_text(&mut self, text: String) -> &mut Tts {
        self.text = text;
        self
    }

    /// Configure et exécute le moteur TTS
    pub fn speak(&mut self, engine: &mut impl TtsEngine) -> &mut Tts {
        engine
            .set_lang(self.lang.clone())
            .set_speed(self.speed)
            .speak(&self.text);

        self
    }

    /// Joue le fichier audio généré
    pub fn play(&self, engine: impl PlayEngine) {
        engine.play(self.path.as_str());
    }

    /// Arrête la lecture en cours
    pub fn stop(&self, engine: impl PlayEngine) {
        engine.stop();
    }

    /// Liste les moteurs TTS disponibles sur le système
    pub fn list_available_engines() -> Vec<&'static str> {
        let engines = ["pico", "espeak", "espeak-ng"];

        engines
            .iter()
            .filter(|&&engine| match engine {
                "pico" => command_exists("pico2wave"),
                "espeak" => command_exists("espeak"),
                "espeak-ng" => command_exists("espeak-ng"),
                _ => false,
            })
            .cloned()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tts_new() {
        let tts = Tts::new();
        assert_eq!(tts.lang, "fr-FR");
        assert_eq!(tts.speed, 1);
    }

    #[test]
    fn test_tts_set_lang() {
        let mut tts = Tts::new();
        tts.set_lang(String::from("en-US"));
        assert_eq!(tts.lang, "en-US");
    }

    #[test]
    fn test_tts_set_speed() {
        let mut tts = Tts::new();
        tts.set_speed(150);
        assert_eq!(tts.speed, 150);
    }

    #[test]
    fn test_tts_set_text() {
        let mut tts = Tts::new();
        tts.set_text(String::from("Hello"));
        assert_eq!(tts.text, "Hello");
    }

    #[test]
    fn test_tts_builder_pattern() {
        let mut tts = Tts::new();
        tts.set_lang(String::from("fr-FR"))
            .set_speed(100)
            .set_text(String::from("Bonjour"));

        assert_eq!(tts.lang, "fr-FR");
        assert_eq!(tts.speed, 100);
        assert_eq!(tts.text, "Bonjour");
    }
}
