mod cli;
mod input;
mod player;
mod translate;
mod tts;
mod utils;

use input::Input;
use translate::Translate;
use tts::TtsEgine;
use utils::get_pidof;
use utils::textutils::read_vars;

use crate::player::paplay::Paplay;
use crate::tts::espeak::Espeak;
use crate::tts::pico::Pico;
use crate::tts::Tts;
use crate::utils::textutils::text_to_dict;

fn main() {
    let args = cli::build_app();

    let mut tts = Tts::new();

    if *args.get_one::<bool>("stop").unwrap() {
        tts.stop(Paplay {});
        return;
    }

    if get_pidof("gsp").len() > 1 {
        println!("Une autre instance du programme est en cours");
        tts.stop(Paplay {});
        return;
    }

    if !args.contains_id("source") {
        println!("Aucune source n'a été spécifiée");
        return;
    }

    let mut lang_sources = args.get_one::<String>("lang_targets").unwrap();
    if args.contains_id("lang_sources") {
        lang_sources = args.get_one::<String>("lang_sources").unwrap();
    }

    let mut text = Input::new()
        .set_source(args.get_one::<String>("source").unwrap().clone())
        .set_lang(lang_sources.clone())
        .input();

    if text.is_empty() {
        println!("Aucun texte à lire");
        return;
    }

    if *args.get_one::<bool>("dev").unwrap() {
        text = read_vars(&text);
    }

    text = text_to_dict(&text);

    let lang_targets = args.get_one::<String>("lang_targets").unwrap();

    if args.contains_id("lang_sources") {
        let engine = args.get_one::<String>("engine-translation").unwrap();

        text = Translate {}.translate(
            engine.as_str(),
            &text,
            args.get_one::<String>("lang_sources").unwrap().as_str(),
            args.get_one::<String>("lang_targets").unwrap().as_str(),
        );
    }

    let speed = args.get_one::<String>("speed").unwrap();
    let speed = speed.parse::<f32>().unwrap() * 100.0;
    let speed = speed as i32;

    tts.set_lang(lang_targets.to_string())
        .set_speed(speed)
        .set_text(text);

    let engine = args.get_one::<String>("engine-tts").unwrap();

    match engine.as_str() {
        "pico" => tts.speak(&mut Pico::new()),
        "espeak" => tts.speak(&mut Espeak::new()),
        _ => tts.speak(&mut Pico::new()),
    }
    tts.play(Paplay {});
}
