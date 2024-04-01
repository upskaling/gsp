use std::process::{Command, Stdio};

// tesseract /tmp/screenshot.png -l fra stdout
pub fn tesseract(screenshooter: &str, lang: &str) -> String {
    let lang = match lang {
        "de-DE" => "deu",
        "en-GB" => "eng",
        "es-ES" => "spa",
        "fr-FR" => "fra",
        "it-IT" => "ita",
        _ => "eng",
    };

    let command = Command::new("tesseract")
        .arg(screenshooter)
        .arg("stdout")
        .arg("-l")
        .arg(lang)
        .stderr(Stdio::null())
        .output()
        .expect("failed to execute process");

    let stdout = String::from_utf8_lossy(&command.stdout);

    stdout.to_string()
}
