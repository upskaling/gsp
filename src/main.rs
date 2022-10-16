use crate::clipboard::get_clipboard;
use crate::selection::get_selection;
use crate::textutils::text_to_dict;
use crate::tts::espeak::Espeak;
use crate::tts::paplay::Paplay;
use crate::tts::pico::Pico;
use crate::tts::tts::Tts;

mod clipboard;
mod selection;
mod textutils;
mod translate;
mod tts;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    // selection
    #[clap(long, help = "Read text from selection")]
    selection: bool,

    // clipboard
    #[clap(short, long, help = "Read text from clipboard")]
    clipboard: bool,

    // ocr
    #[clap(short, long, help = "Read text from image")]
    ocr: bool,

    // stdin
    #[clap(long, help = "Read text from stdin")]
    stdin: bool,

    // input-file
    #[clap(long, help = "Read text from file")]
    input_file: Option<String>,

    // input-text
    #[clap(long, help = "Read text from argument")]
    input_text: Option<String>,

    // tts {pico,espeak,spd-say}
    #[clap(
        short = 'y',
        long,
        help = "Read text using specified TTS engine",
        default_value = "pico"
    )]
    tts: Option<String>,

    // lang [{de-DE,en-GB,en-US,es-ES,fr-FR,it-IT}]
    #[clap(
        short,
        long,
        help = "Set language for TTS engine",
        default_value = "fr-FR"
    )]
    lang: Option<String>,

    // translation [{de-DE,en-GB,en-US,es-ES,fr-FR,it-IT}]
    #[clap(short, long, help = "Language to source translation from")]
    translation: Option<String>,

    // engine-trans [{libretranslate,argos_translate,translate_shell}]
    #[clap(
        short,
        long,
        help = "Translation engine to use",
        default_value = "libretranslate"
    )]
    engine_trans: Option<String>,

    // speed [{0.6,0.8,1,1.2,1.4,1.6,1.8,2,2.2}]
    #[clap(long, help = "Set speed for TTS engine", default_value = "1")]
    speed: Option<String>,

    // output-file
    #[clap(long)]
    output_file: Option<String>,

    // stop
    #[clap(short, long)]
    stop: bool,
}

fn main() {
    let args = Args::parse();

    let mut tts = Tts::new();
    if args.stop {
        tts.stop(Paplay {});
        return;
    }

    // Vérifier que le programme a été lancé une seule fois lock

    let mut text = String::new();
    // Récupérer la sélection
    if args.selection {
        text = get_selection();
    } else if args.clipboard {
        text = get_clipboard();
    }

    if text == "" {
        println!("No text to read");
        return;
    }

    text = text_to_dict(&text);

    if args.translation.is_some() {
        use crate::translate::translate_engine::TranslateEngine;

        let engine = args.engine_trans.unwrap();
        if engine == "libretranslate" {
            use crate::translate::libretranslate::Libretranslate;

            let translate = Libretranslate {};
            text = translate.translate(
                &text,
                &args.translation.unwrap(),
                &args.lang.as_ref().unwrap(),
            );
        } else if engine == "argos_translate" {
            use crate::translate::argos_translate::ArgosTranslate;

            let translate = ArgosTranslate {};
            text = translate.translate(
                &text,
                &args.translation.unwrap(),
                &args.lang.as_ref().unwrap(),
            );
        }

        println!("{}", text);
    }

    tts.set_lang(args.lang.unwrap())
        .set_speed(args.speed.unwrap());

    match args.tts.unwrap().as_str() {
        "pico" => tts.speak(Pico {}, &text),
        "espeak" => tts.speak(Espeak {}, &text),
        _ => tts.speak(Pico {}, &text),
    }

    tts.play(Paplay {});
}
