use std::process::{Command, Stdio};

// gnome-screenshot --area --file=/tmp/screenshot.png
pub fn gnome_screenshot_area(screenshooter: &str) {
    Command::new("gnome-screenshot")
        .arg("--area")
        .arg(format!("--file={}", screenshooter))
        .stdout(Stdio::piped())
        .output()
        .expect("failed to execute process");
}
