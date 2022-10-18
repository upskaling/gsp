mod cli;
mod input;
mod player;
mod textutils;
mod translate;
mod tts;
mod utils;

use tts::tts::TtsEgine;
use utils::get_pid;

use crate::input::clipboard::Clipboard;
use crate::input::selection::Selection;
use crate::input::InputEngine;
use crate::player::paplay::Paplay;
use crate::textutils::text_to_dict;
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

    if get_pid("gsp") {
        println!("Une autre instance du programme est en cours");
        tts.stop(Paplay {});
        return;
    }

    let mut text = String::new();
    // Récupérer la sélection
    if *args.get_one::<bool>("selection").unwrap() {
        let selection = Selection {};
        text = selection.input();
    } else if *args.get_one::<bool>("clipboard").unwrap() {
        let clipboard = Clipboard {};
        text = clipboard.input();
    }

    if text == "" {
        println!("No text to read");
        return;
    }

    text = text_to_dict(&text);

    let lang = args.get_one::<String>("lang").unwrap();

    if args.contains_id("translation") {
        let translation = args.get_one::<String>("translation").unwrap();
        let engine = args.get_one::<String>("engine-translation").unwrap();

        if engine == "libretranslate" {
            text = Libretranslate {}.translate(&text, translation.as_str(), lang);
        } else if engine == "argos_translate" {
            text = ArgosTranslate {}.translate(&text, translation.as_str(), lang);
        }
    }

    let speed = args.get_one::<String>("speed").unwrap();
    let speed = speed.parse::<f32>().unwrap() * 100.0;
    let speed = speed as i32;

    tts.set_lang(lang.clone()).set_speed(speed);

    let engine = args.get_one::<String>("engine-tts").unwrap();

    if engine == "pico" {
        tts.speak(&text, &mut Pico::new());
    } else if engine == "espeak" {
        tts.speak(&text, &mut Espeak::new());
    }

    tts.play(Paplay {});
}
