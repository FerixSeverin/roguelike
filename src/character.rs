use crate::item;
use bevy::prelude::{Entity, Commands};
use bevy::ecs::Query;

pub struct Player;

pub struct Evil;

#[derive(Default)]
pub struct Inventory {
    pub items: Vec<Entity>
}

impl Inventory {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn starting_inventory(items: Vec<Entity>) -> Self {
        Inventory {
            items
        }
    }

    pub fn add(&mut self, item: Entity) {
        self.items.push(item);
    }

    pub fn remove(&mut self, index: usize) {
        self.items.remove(index);
    }

    pub fn look(&self, item_components: &Query<(&item::Item)>) {
        for item in &self.items {
            println!("{}", item_components.get(*item).unwrap().name);
        }
    }
}

pub struct Attribute {
    base: i32
}

pub struct Attributes {
    pub health: Attribute,
    pub armor: Attribute,
}

impl Attributes {
    pub fn new(health_base: i32, armor_base: i32) -> Self {
        Attributes {
            health: Attribute { base: health_base },
            armor: Attribute { base: armor_base }
        }
    }
}