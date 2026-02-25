//! Module de reconnaissance OCR avec Tesseract
//!
//! Interface pour l'outil de reconnaissance Tesseract.

use std::process::{Command, Stdio};

/// Exécute Tesseract OCR sur une image
///
/// # Arguments
/// * `screenshooter` - Chemin vers le fichier image
/// * `lang` - Code de langue (ex: "fr-FR", "en-US")
///
/// # Retour
/// Le texte reconnu, ou une chaîne vide en cas d'erreur
pub fn tesseract(screenshooter: &str, lang: &str) -> String {
    let lang = match lang {
        "de-DE" => "deu",
        "en-GB" => "eng",
        "es-ES" => "spa",
        "fr-FR" => "fra",
        "it-IT" => "ita",
        _ => "eng",
    };

    match Command::new("tesseract")
        .arg(screenshooter)
        .arg("stdout")
        .arg("-l")
        .arg(lang)
        .stderr(Stdio::null())
        .output()
    {
        Ok(output) => String::from_utf8_lossy(&output.stdout).to_string(),
        Err(e) => {
            eprintln!("Erreur lors de l'exécution de Tesseract: {}", e);
            String::new()
        }
    }
}
