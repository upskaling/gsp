mod cli;
mod input;
mod player;
mod textutils;
mod translate;
mod tts;

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
use clap::Parser;
use tts::tts::TtsEgine;

fn main() {
    let args = cli::Args::parse();

    let mut tts = Tts::new();
    if args.stop {
        tts.stop(Paplay {});
        return;
    }

    // Vérifier que le programme a été lancé une seule fois lock

    let mut text = String::new();
    // Récupérer la sélection
    if args.selection {
        let selection = Selection {};
        text = selection.input();
    } else if args.clipboard {
        let clipboard = Clipboard {};
        text = clipboard.input();
    }

    if text == "" {
        println!("No text to read");
        return;
    }

    text = text_to_dict(&text);

    if args.translation.is_some() {
        let engine = args.engine_trans.unwrap();

        if engine == "libretranslate" {
            text = Libretranslate {}.translate(
                &text,
                &args.translation.unwrap(),
                &args.lang.as_ref().unwrap(),
            );
        } else if engine == "argos_translate" {
            text = ArgosTranslate {}.translate(
                &text,
                &args.translation.unwrap(),
                &args.lang.as_ref().unwrap(),
            );
        }
    }

    tts.set_lang(args.lang.unwrap())
        .set_speed(args.speed.unwrap().parse::<i32>().unwrap());

    let engine = args.tts.unwrap();

    if engine == "pico" {
        tts.speak(&text, &mut Pico::new());
    } else if engine == "espeak" {
        tts.speak(&text, &mut Espeak::new());
    }

    tts.play(Paplay {});
}
