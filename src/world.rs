use ron::de::from_reader;
use serde::Deserialize;
use std::fs::File;
use bevy::prelude::Entity;
use std::collections::HashMap;

pub struct Tile {
    pub main: Option<Entity>,
    pub temperature: i16,
    pub air: Option<Entity>,
    pub ground: Option<Entity>,
    pub base: Option<Entity>,
}

pub struct World {
    pub grid: HashMap<(i32, i32), Tile>,
}

impl World {
    pub fn new() -> Self {
        Self {
            grid: HashMap::<(i32, i32), Tile>::new()
        }
    }

    pub fn move_main(&mut self, first_position: &(i32, i32), second_position: &(i32, i32)) {
        if self.grid.contains_key(&first_position) && self.grid.contains_key(&second_position) {
            self.grid.get_mut(&second_position).unwrap().main = Option::from(self.grid.get_mut(&first_position).unwrap().main);
            self.grid.get_mut(&first_position).unwrap().clear_main();
        } else {
            println!("Keys do not exist");
        }
    }
}

impl Tile {
    pub fn new (main: Entity, temperature: i16, air: Entity, ground: Entity, base: Entity) -> Self {
        Self {
            main: Option::from(main),
            temperature,
            air: Option::from(air),
            ground: Option::from(ground),
            base: Option::from(base),
        }
    }

    pub fn main (entity: Entity) -> Self {
        Self {
            main: Option::from(entity),
            temperature: 0,
            air: None,
            ground: None,
            base: None,
        }
    }

    pub fn clear_main(&mut self) {
        self.main = None;
    }

    pub fn base (entity: Entity) -> Self {
        Self {
            main: None,
            temperature: 0,
            air: None,
            ground: None,
            base: Option::from(entity),
        }
    }

    pub fn empty() -> Self {
        Self {
            main: None,
            temperature: 0,
            air: None,
            ground: None,
            base: None,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct WorldFile {
    main_layer: Vec<String>,
}

impl WorldFile {
    pub fn get(&self) -> &[String] {
        &self.main_layer
    }
}

pub fn load(input_path: String) -> WorldFile {
    let f = File::open(&input_path).expect("Failed opening file");
    let world: WorldFile = match from_reader(f) {
        Ok(x) => x,
        Err(e) => {
            println!("Failed to load the world: {}", e);

            std::process::exit(1);
        }
    };
    world
}
