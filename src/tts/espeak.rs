//! Moteur de synthèse vocale eSpeak
//!
//! Implémentation du trait TtsEngine pour eSpeak.

use std::process::Command;

use super::TtsEngine;

/// Configuration du moteur eSpeak
#[derive(Debug, Clone)]
pub struct Espeak {
    lang: String,
    speed: i32,
    pitch: i32,
    amplitude: i32,
    output_file: String,
}

impl Default for Espeak {
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

impl Espeak {
    /// Crée une nouvelle configuration eSpeak avec des valeurs par défaut
    pub fn new() -> Self {
        Self::default()
    }
}

impl TtsEngine for Espeak {
    fn speak(&self, text: &str) {
        let speed = (self.speed / 100 * 320) / 2;

        let result = Command::new("espeak")
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
            .output();

        if let Err(e) = result {
            eprintln!("Erreur lors de l'exécution d'eSpeak: {}", e);
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
