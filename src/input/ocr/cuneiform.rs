use std::process::Command;

// cuneiform -l fra /tmp/screenshot.png
pub fn cuneiform(screenshooter: &str, lang: &str) -> String {
    let screenshot = "/dev/shm/screenshot.txt";

    let mut child = Command::new("cuneiform")
        .arg("-l")
        .arg(lang)
        .arg("-o")
        .arg(screenshot)
        .arg(screenshooter)
        .spawn()
        .expect("failed to execute process");
    child.wait().expect("failed to wait on child");

    let output = std::fs::read_to_string(screenshot)
        .unwrap()
        .trim()
        .to_string()
        .replace('\n', " ")
        .replace('\r', " ");

    std::fs::remove_file(screenshot).unwrap();

    output
}
