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
use crate::translate::argos_translate::ArgosTranslate;
use crate::translate::libretranslate::Libretranslate;
use crate::translate::translate_engine::TranslateEngine;
use crate::tts::espeak::Espeak;
use crate::tts::pico::Pico;
use crate::tts::tts::Tts;
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

    let source = args.get_one::<String>("source").unwrap();

    let mut text = match source.as_str() {
        "selection" => input::selection::Selection {}.input(),
        "clipboard" => input::clipboard::Clipboard {}.input(),
        "stdin" => input::stdin::Stdin {}.input(),
        "ocr" => input::ocr::Ocr {
            lang: lang_sources.to_string(),
        }
        .input(),
        _ => String::new(),
    };

    if text.is_empty() {
        println!("Aucun texte à lire");
        return;
    }

    text = text_to_dict(&text);

    let lang_targets = args.get_one::<String>("lang_targets").unwrap();

    if args.contains_id("lang_sources") {
        let engine = args.get_one::<String>("engine-translation").unwrap();

        let translate: Box<dyn TranslateEngine> = match engine.as_str() {
            "libretranslate" => Box::new(Libretranslate {}),
            "argos_translate" => Box::new(ArgosTranslate {}),
            _ => Box::new(Libretranslate {}),
        };

        text = translate.translate(&text, &lang_sources, &lang_targets);
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
