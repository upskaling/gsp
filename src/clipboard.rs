use cli_clipboard::{ClipboardContext, ClipboardProvider};

pub fn get_clipboard() -> String {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    let clipboard = ctx.get_contents().unwrap();
    
    clipboard
}
