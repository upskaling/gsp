use clap::{Arg, ArgAction, ArgMatches, Command};

pub fn build_app() -> ArgMatches {
    let app = Command::new("gsp")
        .version("0.1.0")
        .about("Read text from selection, clipboard, image or file")
        .long_about("Read text from selection, clipboard, image or file")
        // selection
        .arg(
            Arg::new("selection")
                .short('s')
                .long("selection")
                .help("Read text from selection")
                .action(ArgAction::SetTrue),
        )
        // clipboard
        .arg(
            clap::Arg::new("clipboard")
                .short('c')
                .long("clipboard")
                .help("Read text from clipboard")
                .action(ArgAction::SetTrue),
        )
        // ocr
        .arg(
            clap::Arg::new("ocr")
                .short('o')
                .long("ocr")
                .help("Read text from image")
                .action(ArgAction::SetTrue),
        )
        // stdin
        .arg(
            clap::Arg::new("stdin")
                .long("stdin")
                .help("Read text from stdin")
                .action(ArgAction::SetTrue),
        )
        // input-file
        .arg(
            clap::Arg::new("input-file")
                .long("input-file")
                .help("Read text from file"),
        )
        // input-text
        .arg(
            clap::Arg::new("input-text")
                .long("input-text")
                .help("Read text from argument"),
        )
        // tts [{pico,espeak,spd-say}]
        .arg(
            clap::Arg::new("engine-tts")
                .short('y')
                .long("tts")
                .help("Read text using specified TTS engine")
                .default_value("pico")
                .value_parser(["pico", "espeak", "spd-say"]),
        )
        // lang [{de-DE,en-GB,en-US,es-ES,fr-FR,it-IT}]
        .arg(
            clap::Arg::new("lang")
                .short('l')
                .long("lang")
                .help("Set language for TTS engine")
                .default_value("fr-FR")
                .value_parser(["de-DE", "en-GB", "en-US", "es-ES", "fr-FR", "it-IT"]),
        )
        // translation [{de-DE,en-GB,en-US,es-ES,fr-FR,it-IT}]
        .arg(
            clap::Arg::new("translation")
                .short('t')
                .long("translation")
                .help("Language to source translation from")
                // .default_value("en-US")
                .value_parser(["de-DE", "en-GB", "en-US", "es-ES", "fr-FR", "it-IT"]),
        )
        // engine-translation
        .arg(
            clap::Arg::new("engine-translation")
                .short('e')
                .long("engine-translation")
                .help("Translation engine to use")
                .default_value("libretranslate")
                .value_parser(["libretranslate", "argos_translate", "translate_shell"]),
        )
        // speed [{0.6,0.8,1,1.2,1.4,1.6,1.8,2,2.2}]
        .arg(
            clap::Arg::new("speed")
                .long("speed")
                .help("Set speed for TTS engine")
                .default_value("1")
                .value_parser(["0.6", "0.8", "1", "1.2", "1.4", "1.6", "1.8", "2", "2.2"]),
        )
        // output-file
        .arg(
            clap::Arg::new("output-file")
                .long("output-file")
                .help("Write output to file"),
        )
        // stop
        .arg(
            clap::Arg::new("stop")
                .short('p')
                .long("stop")
                .help("Stop TTS engine")
                .action(ArgAction::SetTrue),
        );

    app.get_matches()
}
