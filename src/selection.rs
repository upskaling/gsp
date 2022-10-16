use std::process::{Command, Stdio};

// xclip -o
pub fn get_selection() -> String {
    let output = Command::new("xclip")
        .arg("-o")
        .stdout(Stdio::piped())
        .output()
        .expect("failed to execute process");

    let clipboard = String::from_utf8_lossy(&output.stdout).to_string();

    clipboard
}
