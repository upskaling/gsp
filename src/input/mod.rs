mod clipboard;
mod ocr;
mod selection;
mod stdin;

pub trait InputEngine {
    fn input(&self) -> String;
}

pub struct Input {
    pub source: String,
    pub lang: String,
}

impl Input {
    pub fn new() -> Input {
        Input {
            source: String::from(""),
            lang: String::from(""),
        }
    }

    pub fn set_source(&mut self, source: String) -> &mut Input {
        self.source = source;
        self
    }

    pub fn set_lang(&mut self, lang: String) -> &mut Input {
        self.lang = lang;
        self
    }

    pub fn input(&self) -> String {
        match self.source.as_str() {
            "selection" => selection::Selection {}.input(),
            "clipboard" => clipboard::Clipboard {}.input(),
            "stdin" => stdin::Stdin {}.input(),
            "ocr" => ocr::Ocr {
                lang: self.lang.clone(),
            }
            .input(),
            _ => String::new(),
        }
    }
}
