use clap::{Arg, ArgAction, ArgMatches, Command};

pub fn build_app() -> ArgMatches {
    let app = Command::new("gsp")
        .version("0.1.0")
        .about("Read text from selection, clipboard, image or file")
        .long_about("Read text from selection, clipboard, image or file")
        // [selection,clipboard,image,file,ocr]
        .arg(
            Arg::new("source")
                .short('s')
                .long("source")
                .help("Source of text to read")
                .value_parser(["selection", "clipboard", "file", "ocr", "stdin"]),
        )
        // tts [pico,espeak,spd-say]
        .arg(
            clap::Arg::new("engine-tts")
                .short('y')
                .long("tts")
                .help("Read text using specified TTS engine")
                .default_value("pico")
                .value_parser(["pico", "espeak", "spd-say"]),
        )
        // lang [de-DE,en-GB,en-US,es-ES,fr-FR,it-IT]
        .arg(
            clap::Arg::new("lang")
                .short('l')
                .long("lang")
                .help("Set language for TTS engine")
                .default_value("fr-FR")
                .value_parser(["de-DE", "en-GB", "en-US", "es-ES", "fr-FR", "it-IT"]),
        )
        // translation [de-DE,en-GB,en-US,es-ES,fr-FR,it-IT]
        .arg(
            clap::Arg::new("translation")
                .short('t')
                .long("translation")
                .help("Language to source translation from")
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
