//! Module d'utilitaires pour GSP
//!
//! Fournit des fonctions utilitaires pour la gestion des processus et commandes.

use std::process::Command;

use psutil::process::{Process, ProcessError, processes};

pub mod textutils;

/// Récupère les PID des processus correspondant au nom donné
///
/// # Exemples
/// ```
/// let pids = get_pidof("gsp");
/// assert!(pids.len() >= 0);
/// ```
pub fn get_pidof(process_name: &str) -> Vec<u32> {
    let processes = match processes() {
        Ok(p) => p.into_iter(),
        Err(e) => {
            eprintln!("Erreur lors de la récupération des processus: {}", e);
            return Vec::new();
        }
    };

    processes.filter_map(|p| get_pid(p, process_name)).collect()
}

/// Extrait le PID d'un processus si son nom correspond
fn get_pid(process: Result<Process, ProcessError>, process_name: &str) -> Option<u32> {
    let p = match process {
        Ok(p) => p,
        Err(_) => return None,
    };

    match p.name() {
        Ok(name) if name == process_name => Some(p.pid()),
        Ok(_) | Err(_) => None,
    }
}

/// Vérifie si une commande existe dans le PATH
///
/// # Exemples
/// ```
/// assert!(command_exists("ls"));
/// ```
pub fn command_exists(cmd: &str) -> bool {
    Command::new("which")
        .arg(cmd)
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}
