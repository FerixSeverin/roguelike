// Components

pub struct Item {
    pub name: String,
    pub description: String,
}

impl Item {
    pub fn new(name: &str, description: &str) -> Self {
        Item {
            name: name.to_string(),
            description: description.to_string()
        }
    }
}

pub struct Melee {
    pub damage: Damage
}

impl Melee {
    pub fn new(damage_base: i32) -> Self {
        Melee {
            damage: Damage { base: damage_base }
        }
    }
}

pub struct Damage {
    pub base: i32
}