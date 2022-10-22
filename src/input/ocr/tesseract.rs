use std::process::{Command, Stdio};

// tesseract -l fra /tmp/screenshot.png /dev/shm/screenshot
pub fn tesseract(screenshooter: &str, lang: &str) -> String {
    let screenshot = "/dev/shm/screenshot";
    Command::new("tesseract")
        .arg("-l")
        .arg(lang)
        .arg(screenshooter)
        .arg(screenshot)
        .stderr(Stdio::null())
        .output()
        .expect("failed to execute process");

    // lire le fichier /dev/shm/screenshot.txt
    let output = std::fs::read_to_string(format!("{}.txt", screenshot))
        .unwrap()
        .trim()
        .to_string()
        .replace("\n", " ")
        .replace("\r", " ");

    std::fs::remove_file(format!("{}.txt", screenshot)).unwrap();

    output
}
