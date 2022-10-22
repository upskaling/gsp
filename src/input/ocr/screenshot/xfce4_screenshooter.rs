use std::process::{Command, Stdio};

// Take a screenshot of a region of the screen
// xfce4-screenshooter --region --save /tmp/screenshot.png
pub fn xfce4_screenshooter_region(screenshooter: &str) {
    Command::new("xfce4-screenshooter")
        .arg("--region")
        .arg("--save")
        .arg(screenshooter)
        .stdout(Stdio::piped())
        .output()
        .expect("failed to execute process");
}
