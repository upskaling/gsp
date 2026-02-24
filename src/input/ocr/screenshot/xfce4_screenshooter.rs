//! Module de capture d'écran avec xfce4-screenshooter
//!
//! Interface pour l'outil de capture xfce4-screenshooter.

use std::process::{Command, Stdio};

/// Capture une région de l'écran avec xfce4-screenshooter
///
/// # Arguments
/// * `screenshooter` - Chemin où sauvegarder la capture
pub fn xfce4_screenshooter_region(screenshooter: &str) {
    let result = Command::new("xfce4-screenshooter")
        .arg("--region")
        .arg("--save")
        .arg(screenshooter)
        .stdout(Stdio::piped())
        .output();

    if let Err(e) = result {
        eprintln!(
            "Erreur lors de la capture d'écran (xfce4-screenshooter): {}",
            e
        );
    }
}
