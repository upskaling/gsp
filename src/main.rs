mod cli;
mod input;
mod player;
mod translate;
mod tts;
mod utils;

use clap::Parser;
use cli::Args;
use input::Input;
use player::paplay::Paplay;
use tts::{espeak::Espeak, pico::Pico, Tts, TtsEgine};
use utils::{get_pidof, textutils::*};

fn main() {
    let args = Args::parse();

    let mut tts = Tts::new();

    if args.stop {
        tts.stop(Paplay {});
        return;
    }

    if get_pidof("gsp").len() > 1 {
        println!("Une autre instance du programme est en cours");
        tts.stop(Paplay {});
        return;
    }

    let mut text = Input::new(
        args.source,
        args.lang_sources
            .clone()
            .unwrap_or(args.lang_targets.clone()),
    )
    .input();

    if text.is_empty() {
        println!("Aucun texte à lire");
        return;
    }

    if args.dev {
        text = read_vars(&text);
    }

    text = parse_hashtag(&text);
    text = trim_whitespace(&text);
    text = remove_special_characters(&text);

    if args.lang_sources.is_some() {
        text = translate::Translate::new().translate(
            args.engine_translation.as_str(),
            text.as_str(),
            args.lang_sources.as_ref().unwrap().as_str(),
            args.lang_targets.as_str(),
        );
    }

    let speed = args.speed.as_str();
    let speed = speed.parse::<f32>().unwrap() * 100.0;
    let speed = speed as i32;

    tts.set_lang(args.lang_targets.to_string())
        .set_speed(speed)
        .set_text(text);

    match args.engine_tts.as_str() {
        "espeak" => tts.speak(&mut Espeak::new()),
        "pico" => tts.speak(&mut Pico::new()),
        _ => tts.speak(&mut Pico::new()),
    }
    .play(Paplay {});
}
