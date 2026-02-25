//! Module de lecture depuis l'entrée standard (stdin)
//!
//! Implémentation du trait InputEngine pour stdin.

use super::InputEngine;
use std::io::{self, BufRead};

/// Lecteur de l'entrée standard
pub struct Stdin {}

impl InputEngine for Stdin {
    fn input(&self) -> String {
        let mut input = String::new();
        let stdin = io::stdin();

        for line in stdin.lock().lines() {
            match line {
                Ok(l) => {
                    input.push_str(&l);
                    input.push('\n');
                }
                Err(e) => {
                    eprintln!("Erreur lors de la lecture de stdin: {}", e);
                    break;
                }
            }
        }

        input
    }
}
