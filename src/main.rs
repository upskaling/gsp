//! Point d'entrée principal de l'application GSP
//!
//! GSP est un lecteur d'écran avec reconnaissance automatique de langue.

mod cli;
mod error;
mod input;
mod player;
mod translate;
mod tts;
mod utils;

use clap::Parser;
use cli::Args;
use error::{GspError, GspResult};
use input::Input;
use player::rodio::Rodio;
use tts::{Tts, espeak::Espeak, pico::Pico};
use utils::{get_pidof, textutils::*};

use crate::tts::espeakng::EspeakNg;

fn main() {
    if let Err(e) = run() {
        eprintln!("Erreur: {}", e);
        std::process::exit(1);
    }
}

/// Fonction principale exécutant la logique de l'application
fn run() -> GspResult<()> {
    let args = Args::parse();

    if args.stop {
        stop_tts();
        return Ok(());
    }

    if is_another_instance_running() {
        println!("Une autre instance du programme est en cours");
        stop_tts();
        return Ok(());
    }

    let text = get_input_text(&args)?;
    let text = preprocess_text(&args, text);

    let translated_text = if let Some(ref lang_sources) = args.lang_sources {
        translate_text(&args, lang_sources, text)?
    } else {
        text
    };

    let mut tts = configure_tts(&args, translated_text);

    match args.engine_tts.as_str() {
        "espeak" => tts.speak(&mut Espeak::new()),
        "espeak-ng" => tts.speak(&mut EspeakNg::new()),
        "pico" => tts.speak(&mut Pico::new()),
        _ => tts.speak(&mut Pico::new()),
    }
    .play(Rodio {});

    Ok(())
}

fn stop_tts() {
    Tts::new().stop(Rodio {});
}

fn is_another_instance_running() -> bool {
    get_pidof("gsp").len() > 1
}

/// Récupère le texte d'entrée selon la source spécifiée
fn get_input_text(args: &Args) -> GspResult<String> {
    let text = Input::new(
        args.source.clone(),
        args.lang_sources
            .clone()
            .unwrap_or_else(|| args.lang_targets.clone()),
    )
    .input();

    if text.is_empty() {
        return Err(GspError::TextRetrieval("Aucun texte à lire".to_string()));
    }

    Ok(text)
}

/// Prétraite le texte (nettoyage, formatage)
fn preprocess_text(args: &Args, text: String) -> String {
    let mut text = if args.dev {
        read_vars(&text).to_lowercase()
    } else {
        text
    };

    text = parse_hashtag(&text);
    text = trim_whitespace(&text);
    text = remove_special_characters(&text);

    text
}

/// Traduit le texte selon les paramètres
fn translate_text(args: &Args, lang_sources: &str, text: String) -> GspResult<String> {
    translate::Translate::new()
        .translate(
            args.engine_translation.as_str(),
            text.as_str(),
            lang_sources,
            args.lang_targets.as_str(),
        )
        .map_err(|e| GspError::Translation(e.to_string()))
}

/// Configure le moteur TTS avec les paramètres
fn configure_tts(args: &Args, text: String) -> Tts {
    let speed = args.speed.parse::<f32>().unwrap_or(1.0) * 100.0;
    let speed = speed as i32;

    let mut tts = Tts::new();
    tts.set_lang(args.lang_targets.to_string())
        .set_speed(speed)
        .set_text(text);

    tts
}
