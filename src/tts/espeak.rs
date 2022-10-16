use crate::tts::tts::TtsEgine;
use std::process::{Command, Stdio};

pub struct Espeak {}

impl TtsEgine for Espeak {
    fn speak(&self, text: &str) {
        Command::new("espeak")
            .arg("-vmb-fr4")
            .arg("-s 320")
            .arg("-p 45")
            .arg("-a 100")
            .arg("-w /dev/shm/out.wav")
            .arg(text)
            .stdout(Stdio::piped())
            .output()
            .expect("failed to execute process");
    
        // io::stdout().write_all(&output.stdout).unwrap();
        // io::stderr().write_all(&output.stderr).unwrap();
    }
}