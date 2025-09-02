pub mod paplay;

pub trait PlayEngine {
    fn play(&self, file: &str);
    fn stop(&self);
}
