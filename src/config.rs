use ron::de::from_reader;
use serde::Deserialize;
use std::fs::File;
use bevy::input::keyboard::KeyCode;

#[derive(Debug, Deserialize)]
pub struct Config {
    window_size: (u32, u32),
    window_title: String,
    fullscreen: bool,
}

impl Config {
    pub fn window_width(&self) -> u32 {
        self.window_size.0
    }
    pub fn window_height(&self) -> u32 {
        self.window_size.1
    }
    pub fn window_title(&self) -> &String {
        &self.window_title
    }
    pub fn fullscreen(&self) -> bool {
        self.fullscreen
    }
}

pub fn load_config(input_path: String) -> Config {
    let f = File::open(&input_path).expect("Failed opening file");
    let config: Config = match from_reader(f) {
        Ok(x) => x,
        Err(e) => {
            println!("Failed to load config: {}", e);

            std::process::exit(1);
        }
    };
    config
}
