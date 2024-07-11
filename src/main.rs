// modification
mod config;

use std::io;
use std::io::BufReader;
use std::fs::File;
use rodio::{Decoder, OutputStream, Sink}; // Decoder, OutputStream and Sink w/ rodio

fn main() {
    let mut music_path = config::get_last_music(); // get last music
    let (stream, stream_handle) = OutputStream::try_default().unwrap(); // create a thread for audio io
    let sink = Sink::try_new(&stream_handle).unwrap(); // for control music
    let mut run: bool = true; // run flag
    let mut music_run: bool = false; // music play flag on/off
    let mut seek_position: Option<u64> = None; // variable to store search position

    while run {
        // main program cycle
        let mut command = String::new(); // var to store command
        let _ = io::stdin().read_line(&mut command); // user input
        let command = command.trim(); // delete spaces

        if command.starts_with("ref ") {
            // processing the music path change command
            let new_music_path = command.trim_start_matches("ref ").trim(); // take new music path
            sink.stop(); // stop music
            sink.clear(); // clear sink
            music_run = false; // set music run to false /off
            music_path = String::from(new_music_path); // update path to music
            println!("Music path set to: {}", music_path); // print new path
            seek_position = None; // rest search position

        } else if command.starts_with("ps ") {
            // processing search command
            let position_str = command.trim_start_matches("ps ").trim(); // take position
            match position_str.parse::<u64>() {
                // parse to u64
                Ok(pos) => {
                    if music_run {
                        // if music run
                        let _ = sink.try_seek(std::time::Duration::from_secs(pos)); // try take search position
                    } else {
                        seek_position = Some(pos); // if music of, save position for future search
                    }
                    println!("Seeking to {} seconds.", pos); // print info about search pos
                },
                Err(_) => {
                    println!("Invalid seconds format."); // except
                }
            }

        } else if command.starts_with("sv ") {
            let set_volume = command.replace("sv ", "");
            let set_volume = set_volume.trim();
            let set_volume: f32 = set_volume.parse().expect("gg");
            sink.set_volume(set_volume);

        } else if command.starts_with("ss ") {
            let set_speed = command.replace("ss ", "");
            let set_speed = set_speed.trim();
            let set_speed: f32 = set_speed.parse().expect("gg");
            sink.set_speed(set_speed);

        } else if command == "play" {
            // processing play command
            if !music_run {
                // if music play
                let file = File::open(&music_path).unwrap(); // open music file
                let source = Decoder::new(BufReader::new(file)).unwrap(); // create decoder for read file
                sink.append(source); // add sink source
                if let Some(pos) = seek_position {
                    let _ = sink.try_seek(std::time::Duration::from_secs(pos)); // set search position
                    seek_position = None; // reset search position
                }
                sink.play(); // start play
                music_run = true; // set music run flag to true /on
            }

        } else if command == "stop" {
            // processing stop command
            if music_run {
                // if music play
                sink.pause(); // stop music
                music_run = false; // set music run flag to false /off
            }

        } else if command == "cs" {
            // set music pos
            let sec = sink.get_pos(); // take now pos
            let sec = sec.as_secs(); // convert to sec
            println!("{}", sec); // print now pos

        } else if command == "!exit" {
            // processing exit
            config::set_last_music(&music_path); // save last music path to config //// config file config.json //// my lib config.rs
            sink.stop(); // stop music
            music_run = false; // set music run flag to false
            run = false; // stop main cycle and off program
        }


    }
}
