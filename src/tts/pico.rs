//! Moteur de synthèse vocale Pico
//!
//! Implémentation du trait TtsEngine pour Pico TTS.

use super::TtsEngine;
use crate::utils::textutils::replace;
use std::process::{Command, Stdio};

/// Applique des effets SSML au texte pour Pico
fn pico_effect(text: &str, speed: i32, pitch: i32, volume: i32) -> String {
    let speed = format!("<speed level=\"{}\">{}</speed>", speed, text);
    let pitch = format!("<pitch level=\"{}\">{}</pitch>", pitch, speed);
    let volume = format!("<volume level=\"{}\">{}</volume>", volume, pitch);
    volume
}

/// Configuration du moteur Pico
#[derive(Debug, Clone)]
pub struct Pico {
    pub lang: String,
    pub speed: i32,
    pub output_file: String,
}

impl Default for Pico {
    fn default() -> Self {
        Self {
            lang: "fr-FR".to_string(),
            speed: 1,
            output_file: "/dev/shm/out.wav".to_string(),
        }
    }
}

impl Pico {
    /// Crée une nouvelle configuration Pico avec des valeurs par défaut
    pub fn new() -> Self {
        Self::default()
    }
}

impl TtsEngine for Pico {
    fn speak(&self, text: &str) {
        let text = replace(text);
        let effect = pico_effect(&text, self.speed, 100, 120);

        let result = Command::new("pico2wave")
            .arg(format!("--lang={}", self.lang))
            .arg(format!("-w={}", self.output_file))
            .arg("--")
            .arg(effect)
            .stdout(Stdio::piped())
            .output();

        if let Err(e) = result {
            eprintln!("Erreur lors de l'exécution de Pico TTS: {}", e);
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
