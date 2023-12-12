use crate::input::InputEngine;
use std::time::Duration;
use x11_clipboard::Clipboard;

pub struct Selection {}

impl InputEngine for Selection {
    fn input(&self) -> String {
        let clipboard = Clipboard::new().unwrap();

        let selection = clipboard
            .load(
                clipboard.setter.atoms.primary,
                clipboard.setter.atoms.utf8_string,
                clipboard.setter.atoms.property,
                Duration::from_secs(3),
            )
            .unwrap();

        String::from_utf8(selection).unwrap()
    }
}
