use crate::player::PlayEngine;
use psutil::process::processes;
pub struct Rodio {}

impl PlayEngine for Rodio {
    fn play(&self, file: &str) {
        let stream_handle = rodio::OutputStreamBuilder::open_default_stream().unwrap();
        let sink = rodio::Sink::connect_new(stream_handle.mixer());

        let file = std::fs::File::open(file).unwrap();
        sink.append(rodio::Decoder::try_from(file).unwrap());

        sink.sleep_until_end();
    }

    fn stop(&self) {
        for process in processes().unwrap() {
            let process = process.unwrap();

            if process.name().unwrap() != "gsp" {
                continue;
            }

            process.kill().unwrap();
            println!("killed paplay process {}", process.pid());
        }
    }
}
