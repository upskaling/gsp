use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    // selection
    #[clap(long, help = "Read text from selection")]
    pub selection: bool,

    // clipboard
    #[clap(short, long, help = "Read text from clipboard")]
    pub clipboard: bool,

    // ocr
    #[clap(short, long, help = "Read text from image")]
    pub ocr: bool,

    // stdin
    #[clap(long, help = "Read text from stdin")]
    pub stdin: bool,

    // input-file
    #[clap(long, help = "Read text from file")]
    pub input_file: Option<String>,

    // input-text
    #[clap(long, help = "Read text from argument")]
    pub input_text: Option<String>,

    // tts {pico,espeak,spd-say}
    #[clap(
        short = 'y',
        long,
        help = "Read text using specified TTS engine",
        default_value = "pico"
    )]
    pub tts: Option<String>,

    // lang [{de-DE,en-GB,en-US,es-ES,fr-FR,it-IT}]
    #[clap(
        short,
        long,
        help = "Set language for TTS engine",
        default_value = "fr-FR"
    )]
    pub lang: Option<String>,

    // translation [{de-DE,en-GB,en-US,es-ES,fr-FR,it-IT}]
    #[clap(short, long, help = "Language to source translation from")]
    pub translation: Option<String>,

    // engine-trans [{libretranslate,argos_translate,translate_shell}]
    #[clap(
        short,
        long,
        help = "Translation engine to use",
        default_value = "libretranslate"
    )]
    pub engine_trans: Option<String>,

    // speed [{0.6,0.8,1,1.2,1.4,1.6,1.8,2,2.2}]
    #[clap(long, help = "Set speed for TTS engine", default_value = "1")]
    pub speed: Option<String>,

    // output-file
    #[clap(long)]
    pub output_file: Option<String>,

    // stop
    #[clap(short, long)]
    pub stop: bool,
}
