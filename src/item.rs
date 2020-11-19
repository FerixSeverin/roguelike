// Globals

#[derive(PartialEq)]
pub enum EqiuppedInto {
    Weapon,
    Armor,
}

// Components

pub struct Item {
    pub name: String,
    pub description: String,
    pub weight: u32,
}

impl Item {
    pub fn new(name: &str, description: &str, weight: u32) -> Self {
        Item {
            name: name.to_string(),
            description: description.to_string(),
            weight,
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

pub struct Consumable;

pub struct Equippable {
    pub equipped_into: EqiuppedInto,
}

impl Equippable {
    pub fn new(equipped_into: EqiuppedInto) -> Self {
        Equippable {
            equipped_into,
        }
    }
}