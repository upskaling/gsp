//! Module de capture d'écran pour l'OCR
//!
//! Fournit une interface unifiée pour les différents outils de capture.

mod gnome_screenshot;
mod xfce4_screenshooter;

use self::xfce4_screenshooter::xfce4_screenshooter_region;
use which::which;

/// Capture d'écran pour l'OCR
pub struct Screenshot {
    pub path: String,
}

impl Default for Screenshot {
    fn default() -> Self {
        Self::new()
    }
}

impl Screenshot {
    /// Crée une nouvelle configuration de capture d'écran
    pub fn new() -> Self {
        Screenshot {
            path: "/dev/shm/screenshot.png".to_string(),
        }
    }

    /// Capture une région de l'écran et retourne le chemin du fichier
    pub fn capture(&self) -> String {
        if which("xfce4-screenshooter").is_ok() {
            xfce4_screenshooter_region(&self.path);
        } else if which("gnome-screenshot").is_ok() {
            gnome_screenshot::gnome_screenshot_area(&self.path);
        } else {
            eprintln!("Aucun outil de capture d'écran trouvé");
        }

        self.path.clone()
    }
}
