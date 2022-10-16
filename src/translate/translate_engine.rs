pub trait TranslateEngine {
    fn translate(&self, text: &str, lang_from: &str, lang_to: &str) -> String;
}