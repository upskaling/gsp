pub mod paplay;
pub mod play;

pub trait PlayEngine {
    fn play(&self, file: &str);
    fn stop(&self);
}
