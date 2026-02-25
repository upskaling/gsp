//! Module de reconnaissance OCR avec Cuneiform
//!
//! Interface pour l'outil de reconnaissance Cuneiform.

use std::process::Command;

/// Exécute Cuneiform OCR sur une image
///
/// # Arguments
/// * `screenshooter` - Chemin vers le fichier image
/// * `lang` - Code de langue
///
/// # Retour
/// Le texte reconnu, ou une chaîne vide en cas d'erreur
pub fn cuneiform(screenshooter: &str, lang: &str) -> String {
    let screenshot_output = "/dev/shm/screenshot.txt";

    let mut child = match Command::new("cuneiform")
        .arg("-l")
        .arg(lang)
        .arg("-o")
        .arg(screenshot_output)
        .arg(screenshooter)
        .spawn()
    {
        Ok(child) => child,
        Err(e) => {
            eprintln!("Erreur lors de l'exécution de Cuneiform: {}", e);
            return String::new();
        }
    };

    if let Err(e) = child.wait() {
        eprintln!("Erreur lors de l'attente de Cuneiform: {}", e);
        return String::new();
    }

    match std::fs::read_to_string(screenshot_output) {
        Ok(content) => {
            let output = content.trim().to_string().replace(['\n', '\r'], " ");

            // Nettoyage du fichier temporaire
            if let Err(e) = std::fs::remove_file(screenshot_output) {
                eprintln!(
                    "Avertissement: impossible de supprimer le fichier temporaire: {}",
                    e
                );
            }

            output
        }
        Err(e) => {
            eprintln!("Erreur lors de la lecture du résultat Cuneiform: {}", e);
            String::new()
        }
    }
}
