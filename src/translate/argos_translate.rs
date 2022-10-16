use crate::translate::translate_engine::TranslateEngine;
use std::process::{Command, Stdio};

pub struct ArgosTranslate {}

impl TranslateEngine for ArgosTranslate {
    fn translate(&self, text: &str, lang_from: &str, lang_to: &str) -> String {
        let lang_from = lang_from[..2].to_string();
        let lang_to = lang_to[..2].to_string();

        println!("{} -> {}", lang_from, lang_to);

        let output = Command::new("argos-translate-cli")
            .arg("--from-lang")
            .arg(lang_from)
            .arg("--to-lang")
            .arg(lang_to)
            .arg(text)
            .stdout(Stdio::piped())
            .output()
            .expect("failed to execute process");


        let stderr = String::from_utf8_lossy(&output.stderr);
        if stderr != "" {
            println!("stderr: {}", stderr);
        }

        let output = String::from_utf8(output.stdout).unwrap();
        let output = output.trim();

        println!("{} -> {}", text, output);
        output.to_string()
    }
}
