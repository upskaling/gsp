mod cli;
mod input;
mod player;
mod translate;
mod tts;
mod utils;

use clap::Parser;
use cli::Args;
use input::Input;
use player::rodio::Rodio;
use tts::{espeak::Espeak, pico::Pico, Tts};
use utils::{get_pidof, textutils::*};

fn main() {
    let args = Args::parse();

    if args.stop {
        stop_tts();
        return;
    }

    if is_another_instance_running() {
        println!("Une autre instance du programme est en cours");
        stop_tts();
        return;
    }

    let text = match get_input_text(&args) {
        Ok(text) => text,
        Err(e) => {
            eprintln!("Erreur lors de la récupération du texte : {}", e);
            return;
        }
    };

    let text = preprocess_text(&args, text);

    let translated_text = if let Some(ref lang_sources) = args.lang_sources {
        translate_text(&args, lang_sources, text).unwrap()
    } else {
        text
    };

    let mut tts = configure_tts(&args, translated_text);

    match args.engine_tts.as_str() {
        "espeak" => tts.speak(&mut Espeak::new()),
        "pico" => tts.speak(&mut Pico::new()),
        _ => tts.speak(&mut Pico::new()),
    }
    .play(Rodio {});
}

fn stop_tts() {
    Tts::new().stop(Rodio {});
}

fn is_another_instance_running() -> bool {
    get_pidof("gsp").len() > 1
}

fn get_input_text(args: &Args) -> Result<String, String> {
    let text = Input::new(
        args.source.clone(),
        args.lang_sources
            .clone()
            .unwrap_or(args.lang_targets.clone()),
    )
    .input();

    if text.is_empty() {
        return Err("Aucun texte à lire".to_string());
    }

    Ok(text)
}

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

fn translate_text(
    args: &Args,
    lang_sources: &str,
    text: String,
) -> Result<String, Box<dyn std::error::Error>> {
    translate::Translate::new().translate(
        args.engine_translation.as_str(),
        text.as_str(),
        lang_sources,
        args.lang_targets.as_str(),
    )
}

fn configure_tts(args: &Args, text: String) -> Tts {
    let speed = args.speed.parse::<f32>().unwrap_or(1.0) * 100.0;
    let speed = speed as i32;

    let mut tts = Tts::new();
    tts.set_lang(args.lang_targets.to_string())
        .set_speed(speed)
        .set_text(text);

    tts
}
