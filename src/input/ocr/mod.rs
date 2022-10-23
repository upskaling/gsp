mod cuneiform;
mod screenshot;
mod tesseract;

use self::{cuneiform::cuneiform, tesseract::tesseract};
use super::InputEngine;
use which::which;

pub struct Ocr {
    pub lang: String,
}

impl InputEngine for Ocr {
    fn input(&self) -> String {
        let screenshooter = screenshot::Screenshot::new().capture();

        let mut input = String::new();
        if which("tesseract").is_ok() {
            input = tesseract(&screenshooter, &self.lang);
        } else if which("cuneiform").is_ok() {
            input = cuneiform(&screenshooter, &self.lang);
        } else {
            println!("Aucun outil de reconnaissance d'écriture n'est installé");
        }

        std::fs::remove_file(screenshooter).unwrap();

        input
    }
}
