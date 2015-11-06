extern crate tts;
use std::io;
use std::fs;
use std::io::BufRead;

fn main() {
    let mut args = std::env::args();
    args.next().unwrap();
    let filename = args.next();

    let mut tts = tts::Speechifier::new();
    tts.start();

    match filename {
        Some(filename) => {
            let file = fs::File::open(filename).unwrap();
            let buffer = io::BufReader::new(file);
            for line in buffer.lines() {
                tts.queue(line.unwrap());
            }
        },
        None => {
            let stdin = io::stdin();
            for line in stdin.lock().lines() {
                tts.queue(line.unwrap());
            }
        }
    }

    tts.stop();
}
