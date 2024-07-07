use std::fs::File;
use std::io::{Read, Write};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Config {
    last_music: String,
}

pub fn get_last_music() -> String {
    let mut file = File::open("config.json").expect("FILE -> config.json die");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("FILE -> config.json destroyed");
    let config: Config = serde_json::from_str(&contents).expect("JSON die");
    let last_music_value = config.last_music;
    last_music_value
}
pub fn set_last_music(new_path: &str) {
    let mut file = File::open("config.json").expect("FILE -> config.json die");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("FILE -> config.json destroyed");
    let mut config: Config = serde_json::from_str(&contents).expect("JSON die");
    config.last_music = String::from(new_path);
    let updated_contents = serde_json::to_string_pretty(&config).expect("JSON die");
    let mut file = File::create("config.json").expect("can't create file");
    file.write_all(updated_contents.as_bytes()).expect("can't write file");
}
