use ron::de::from_reader;
use serde::Deserialize;
use std::fs::File;

#[derive(Debug, Deserialize)]
pub struct World {
    world: Vec<String>,
}

impl World {
    pub fn get(&self) -> &[String] {
        &self.world
    }
}

pub fn load_world(input_path: String) -> World {
    let f = File::open(&input_path).expect("Failed opening file");
    let world: World = match from_reader(f) {
        Ok(x) => x,
        Err(e) => {
            println!("Failed to load the world: {}", e);

            std::process::exit(1);
        }
    };
    world
}
