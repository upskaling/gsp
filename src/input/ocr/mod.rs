//! Module de reconnaissance optique de caractères (OCR)
//!
//! Fournit une interface unifiée pour les différents moteurs OCR.

mod cuneiform;
mod screenshot;
mod tesseract;

use self::{cuneiform::cuneiform, tesseract::tesseract};
use super::InputEngine;
use which::which;

/// Configuration pour la reconnaissance OCR
pub struct Ocr {
    pub lang: String,
}

impl InputEngine for Ocr {
    fn input(&self) -> String {
        let screenshooter = screenshot::Screenshot::new().capture();

        let mut input = String::new();
        if which("tesseract").is_ok() {
            input = tesseract(&screenshooter, &self.lang);
        } else if which("cuneiform").is_ok() {
            input = cuneiform(&screenshooter, &self.lang);
        } else {
            eprintln!("Aucun outil de reconnaissance d'écriture n'est installé");
        }

        // Nettoyage du fichier temporaire avec gestion d'erreur
        if let Err(e) = std::fs::remove_file(&screenshooter) {
            eprintln!(
                "Avertissement: impossible de supprimer le fichier temporaire: {}",
                e
            );
        }

        input
    }
}
