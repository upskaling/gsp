//! Module d'erreurs personnalisées pour GSP
//!
//! Ce module définit les types d'erreurs utilisés dans l'ensemble du projet.

use std::fmt;
use std::io;

/// Type d'erreur personnalisé pour GSP
#[derive(Debug)]
#[allow(dead_code)]
pub enum GspError {
    /// Erreur lors de la récupération du texte
    TextRetrieval(String),
    /// Erreur d'entrée/sortie
    Io(io::Error),
    /// Erreur de traduction
    Translation(String),
    /// Erreur TTS (Text-To-Speech)
    Tts(String),
    /// Erreur de processus
    Process(String),
    /// Erreur de fichier
    File(String),
    /// Erreur de configuration
    Config(String),
}

impl fmt::Display for GspError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GspError::TextRetrieval(msg) => write!(f, "Erreur de récupération de texte: {}", msg),
            GspError::Io(err) => write!(f, "Erreur I/O: {}", err),
            GspError::Translation(msg) => write!(f, "Erreur de traduction: {}", msg),
            GspError::Tts(msg) => write!(f, "Erreur TTS: {}", msg),
            GspError::Process(msg) => write!(f, "Erreur de processus: {}", msg),
            GspError::File(msg) => write!(f, "Erreur de fichier: {}", msg),
            GspError::Config(msg) => write!(f, "Erreur de configuration: {}", msg),
        }
    }
}

impl std::error::Error for GspError {}

impl From<io::Error> for GspError {
    fn from(err: io::Error) -> Self {
        GspError::Io(err)
    }
}

/// Alias de Result utilisant GspError
pub type GspResult<T> = Result<T, GspError>;
