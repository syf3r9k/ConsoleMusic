mod config;

use std::fs::File;
use std::io::{Read, Write};
use serde::{Deserialize, Serialize};

use std::io;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, Sink};

fn main() {
    let mut music_path = config::get_last_music();
    let (stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    let mut run: bool = true;
    let mut music_run: bool = false;
    let mut seek_position: Option<u64> = None;

    while run {
        let mut command = String::new();
        let _ = io::stdin().read_line(&mut command);
        let command = command.trim();

        if command.starts_with("ref ") {
            let new_music_path = command.trim_start_matches("ref ").trim();
            sink.stop();
            sink.clear();
            music_run = false;
            music_path = String::from(new_music_path);
            println!("Music path set to: {}", music_path);
            seek_position = None; // Сбрасываем позицию перемотки при смене трека
        } else if command.starts_with("ps ") {
            let position_str = command.trim_start_matches("ps ").trim();
            match position_str.parse::<u64>() {
                Ok(pos) => {
                    if music_run {
                        let _ = sink.try_seek(std::time::Duration::from_secs(pos));
                    } else {
                        seek_position = Some(pos);
                    }
                    println!("Seeking to {} seconds.", pos);
                },
                Err(_) => {
                    println!("Invalid seconds format.");
                }
            }
        } else if command == "play" {
            if !music_run {
                let file = File::open(&music_path).unwrap();
                let source = Decoder::new(BufReader::new(file)).unwrap();
                sink.append(source);
                if let Some(pos) = seek_position {
                    let _ = sink.try_seek(std::time::Duration::from_secs(pos));
                    seek_position = None;
                }
                sink.play();
                music_run = true;
            }
        } else if command == "stop" {
            if music_run {
                sink.pause();
                music_run = false;
            }
        } else if command == "!cs" {
            let sec = sink.get_pos();
            let sec = sec.as_secs();
            println!("{}", sec);
        } else if command == "!exit" {
            config::set_last_music(&music_path);
            sink.stop();
            music_run = false;
            run = false;
        }
    }
}
