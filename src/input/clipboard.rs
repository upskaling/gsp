use cli_clipboard::{ClipboardContext, ClipboardProvider};
use crate::input::InputEngine;

pub struct Clipboard {}

impl InputEngine for Clipboard {
    fn input(&self) -> String {
        let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
        let contents = ctx.get_contents().unwrap();
        contents
    }
}

