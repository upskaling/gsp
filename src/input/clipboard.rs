use super::InputEngine;
use cli_clipboard::{ClipboardContext, ClipboardProvider};

pub struct Clipboard {}

impl InputEngine for Clipboard {
    fn input(&self) -> String {
        let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();

        ctx.get_contents().unwrap()
    }
}
