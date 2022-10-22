mod cuneiform;
mod screenshot;
mod tesseract;

use self::{cuneiform::cuneiform, tesseract::tesseract};
use super::InputEngine;
use which::which;

pub struct Ocr {}

impl InputEngine for Ocr {
    fn input(&self) -> String {
        let lang = "fra";

        let screenshooter = screenshot::Screenshot::new().capture();

        let mut input = String::new();
        if which("tesseract").is_ok() {
            input = tesseract(&screenshooter, lang);
        } else if which("cuneiform").is_ok() {
            input = cuneiform(&screenshooter, lang);
        } else {
            println!("Aucun outil de reconnaissance d'écriture n'est installé");
        }

        std::fs::remove_file(screenshooter).unwrap();

        input
    }
}
