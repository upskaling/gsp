//! Module de lecture audio avec Rodio
//!
//! Implémentation du trait PlayEngine pour Rodio.

use crate::player::PlayEngine;
use psutil::process::processes;

/// Lecteur audio basé sur Rodio
pub struct Rodio {}

impl PlayEngine for Rodio {
    fn play(&self, file: &str) {
        let stream_handle = match rodio::OutputStreamBuilder::open_default_stream() {
            Ok(stream) => stream,
            Err(e) => {
                eprintln!("Erreur lors de l'ouverture du flux audio: {}", e);
                return;
            }
        };

        let sink = rodio::Sink::connect_new(stream_handle.mixer());

        let file = match std::fs::File::open(file) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("Erreur lors de l'ouverture du fichier audio: {}", e);
                return;
            }
        };

        match rodio::Decoder::try_from(file) {
            Ok(decoder) => {
                sink.append(decoder);
                sink.sleep_until_end();
            }
            Err(e) => {
                eprintln!("Erreur lors du décodage du fichier audio: {}", e);
            }
        }
    }

    fn stop(&self) {
        let processes = match processes() {
            Ok(p) => p,
            Err(e) => {
                eprintln!("Erreur lors de la récupération des processus: {}", e);
                return;
            }
        };

        for process in processes {
            let process = match process {
                Ok(p) => p,
                Err(_) => continue,
            };

            let name = match process.name() {
                Ok(n) => n,
                Err(_) => continue,
            };

            // Arrêter les processus de lecture audio (gsp)
            if name == "gsp" {
                match process.kill() {
                    Ok(_) => println!(
                        "Processus de lecture arrêté: {} (PID: {})",
                        name,
                        process.pid()
                    ),
                    Err(e) => eprintln!("Erreur lors de l'arrêt du processus: {}", e),
                }
            }
        }
    }
}
