//! Module de lecture du presse-papiers
//!
//! Implémentation du trait InputEngine pour le presse-papiers.

use super::InputEngine;
use cli_clipboard::{ClipboardContext, ClipboardProvider};

/// Lecteur du presse-papiers
pub struct Clipboard {}

impl InputEngine for Clipboard {
    fn input(&self) -> String {
        let mut ctx: ClipboardContext = match ClipboardProvider::new() {
            Ok(ctx) => ctx,
            Err(e) => {
                eprintln!("Erreur lors de l'accès au presse-papiers: {}", e);
                return String::new();
            }
        };

        match ctx.get_contents() {
            Ok(contents) => contents,
            Err(e) => {
                eprintln!("Erreur lors de la lecture du presse-papiers: {}", e);
                String::new()
            }
        }
    }
}
