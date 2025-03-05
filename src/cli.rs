use clap::{ArgAction, Parser};

use crate::translate;

#[derive(Parser, Debug)]
#[command(
    name = "gsp",
    version,
    about = "Read text from selection, clipboard, image or file",
    long_about = "Read text from selection, clipboard, image or file"
)]
pub struct Args {
    #[arg(
        short = 's',
        long,
        help = "Source of text to read",
        value_parser = ["selection", "clipboard", "file", "ocr", "stdin"]
    )]
    pub source: String,

    #[arg(
        short = 'y',
        long = "tts",
        help = "Read text using specified TTS engine",
        default_value = "pico",
        value_parser = ["pico", "espeak"]
    )]
    pub engine_tts: String,

    #[arg(
        short = 't',
        long = "translation",
        help = "Language to source translation from",
        value_parser = ["de-DE", "en-GB", "en-US", "es-ES", "fr-FR", "it-IT", "auto"]
    )]
    pub lang_sources: Option<String>,

    #[arg(
        short = 'e',
        long= "engine-translation",
        help = "Translation engine to use",
        default_value = "libretranslate",
        value_parser =  translate::Translate::list_available_engines()
    )]
    pub engine_translation: String,

    #[arg(
        short = 'l',
        long = "lang",
        help = "Set language for TTS engine",
        default_value = "fr-FR",
        value_parser = ["de-DE", "en-GB", "en-US", "es-ES", "fr-FR", "it-IT"]
    )]
    pub lang_targets: String,

    #[arg(
        long,
        help = "Set speed for TTS engine",
        default_value = "1",
        value_parser = ["0.6", "0.8", "1", "1.2", "1.4", "1.6", "1.8", "2", "2.2"]
    )]
    pub speed: String,

    #[arg(
        short = 'p',
        long,
        help = "Stop TTS engine",
        action = ArgAction::SetTrue
    )]
    pub stop: bool,

    #[arg(
        short,
        long,
        help = "dev mode for natural code reading example: Snake_case, kebab-case, CamelCase",
        action = ArgAction::SetTrue
    )]
    pub dev: bool,
}
