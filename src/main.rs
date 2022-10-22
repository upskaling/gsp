mod cli;
mod input;
mod player;
mod translate;
mod tts;
mod utils;

use tts::tts::TtsEgine;
use utils::get_pidof;

use crate::input::InputEngine;
use crate::player::paplay::Paplay;
use crate::utils::textutils::text_to_dict;
use crate::translate::argos_translate::ArgosTranslate;
use crate::translate::libretranslate::Libretranslate;
use crate::translate::translate_engine::TranslateEngine;
use crate::tts::espeak::Espeak;
use crate::tts::pico::Pico;
use crate::tts::tts::Tts;

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

    let source = args.get_one::<String>("source").unwrap();

    let mut text = match source.as_str() {
        "selection" => input::selection::Selection {}.input(),
        "clipboard" => input::clipboard::Clipboard {}.input(),
        "stdin" => input::stdin::Stdin {}.input(),
        _ => String::new(),
    };

    if text.is_empty() {
        println!("Aucun texte à lire");
        return;
    }

    text = text_to_dict(&text);

    let lang = args.get_one::<String>("lang").unwrap();

    if args.contains_id("translation") {
        let translation = args.get_one::<String>("translation").unwrap();
        let engine = args.get_one::<String>("engine-translation").unwrap();

        text = match engine.as_str() {
            "libretranslate" => Libretranslate {}.translate(&text, translation.as_str(), lang),
            "argos_translate" => ArgosTranslate {}.translate(&text, translation.as_str(), lang),
            _ => String::new(),
        };
    }

    let speed = args.get_one::<String>("speed").unwrap();
    let speed = speed.parse::<f32>().unwrap() * 100.0;
    let speed = speed as i32;

    tts.set_lang(lang.clone()).set_speed(speed).set_text(text);

    let engine = args.get_one::<String>("engine-tts").unwrap();

    match engine.as_str() {
        "pico" => tts.speak(&mut Pico::new()),
        "espeak" => tts.speak(&mut Espeak::new()),
        _ => tts.speak(&mut Pico::new()),
    }
    tts.play(Paplay {});
}
