use std::process::{Command, Stdio};

use crate::utils::command_exists;

use super::TranslateEngine;

pub struct ArgosTranslate {}

impl TranslateEngine for ArgosTranslate {
    fn translate(
        &self,
        text: &str,
        lang_from: &str,
        lang_to: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let lang_from = lang_from[..2].to_string();
        let lang_to = lang_to[..2].to_string();

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
            println!("stderr :: argos-translate ::  {}", stderr);
        }

        let output = String::from_utf8(output.stdout)?;
        let output = output.trim();

        Ok(output.to_string())
    }

    fn is_available(&self) -> bool {
        command_exists("argos-translate-cli")
    }
}
