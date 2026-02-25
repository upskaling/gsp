//! Module de lecture de la sélection X11
//!
//! Implémentation du trait InputEngine pour la sélection X11.

use crate::input::InputEngine;
use std::time::Duration;
use x11_clipboard::Clipboard;

/// Lecteur de la sélection X11
pub struct Selection {}

impl InputEngine for Selection {
    fn input(&self) -> String {
        let clipboard = match Clipboard::new() {
            Ok(cb) => cb,
            Err(e) => {
                eprintln!(
                    "Erreur lors de l'initialisation du presse-papiers X11: {}",
                    e
                );
                return String::new();
            }
        };

        let selection = match clipboard.load(
            clipboard.setter.atoms.primary,
            clipboard.setter.atoms.utf8_string,
            clipboard.setter.atoms.property,
            Duration::from_secs(3),
        ) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Erreur lors de la lecture de la sélection X11: {}", e);
                return String::new();
            }
        };

        match String::from_utf8(selection) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Erreur lors de la conversion UTF-8: {}", e);
                String::new()
            }
        }
    }
}
