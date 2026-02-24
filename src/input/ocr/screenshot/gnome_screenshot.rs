//! Module de capture d'écran avec gnome-screenshot
//!
//! Interface pour l'outil de capture gnome-screenshot.

use std::process::{Command, Stdio};

/// Capture une zone de l'écran avec gnome-screenshot
///
/// # Arguments
/// * `screenshooter` - Chemin où sauvegarder la capture
pub fn gnome_screenshot_area(screenshooter: &str) {
    let result = Command::new("gnome-screenshot")
        .arg("--area")
        .arg(format!("--file={}", screenshooter))
        .stdout(Stdio::piped())
        .output();

    if let Err(e) = result {
        eprintln!(
            "Erreur lors de la capture d'écran (gnome-screenshot): {}",
            e
        );
    }
}
