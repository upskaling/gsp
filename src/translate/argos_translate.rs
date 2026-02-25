//! Module de traduction avec Argos Translate
//!
//! Interface pour l'outil de traduction Argos Translate.

use std::process::{Command, Stdio};

use crate::utils::command_exists;

use super::TranslateEngine;

/// Service de traduction Argos Translate
pub struct ArgosTranslate {}

impl TranslateEngine for ArgosTranslate {
    fn translate(
        &self,
        text: &str,
        lang_from: &str,
        lang_to: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let lang_from = lang_from[..2].to_string();
        let lang_to = lang_to[..2].to_string();

        let output = Command::new("argos-translate-cli")
            .arg("--from-lang")
            .arg(&lang_from)
            .arg("--to-lang")
            .arg(&lang_to)
            .arg(text)
            .stdout(Stdio::piped())
            .output()?;

        let stderr = String::from_utf8_lossy(&output.stderr);
        if !stderr.is_empty() {
            eprintln!("argos-translate: {}", stderr);
        }

        let output = String::from_utf8(output.stdout)?;
        let output = output.trim();

        Ok(output.to_string())
    }

    fn is_available(&self) -> bool {
        command_exists("argos-translate-cli")
    }
}
