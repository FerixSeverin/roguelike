use crate::item;
use bevy::prelude::{Entity, Commands};
use bevy::ecs::Query;

pub struct Player;

pub struct Evil;

pub struct Slot {
    entity: Option<Entity>
}

impl Slot {
    pub fn nothing() -> Self {
        Slot {
            entity: None,
        }
    }

    pub fn equip(&mut self, entity: Entity) {
        self.entity = Option::from(entity);
    }
}

pub struct Equipment {
    pub weapon: Slot,
    pub armor: Slot,
}

impl Equipment {
    pub fn naked() -> Self {
        Equipment {
            weapon: Slot::nothing(),
            armor: Slot::nothing(),
        }
    }

    pub fn look(&self, item_components: &Query<(&item::Item, &item::Equippable)>) {
        if self.armor.entity.is_some() {
            println!("Armor: {}", item_components.get_component::<item::Item>(self.armor.entity.unwrap()).unwrap().name);
        }
        else {
            println!("Armor: None");
        }

        if self.weapon.entity.is_some() {
            println!("Weapon: {}", item_components.get_component::<item::Item>(self.weapon.entity.unwrap()).unwrap().name);
        } else {
            println!("Weapon: None");
        }
    }
}

#[derive(Default)]
pub struct Inventory {
    pub items: Vec<Entity>,
    pub max_weight: u32,
    pub current_weight: u32,
}

impl Inventory {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn starting_inventory(items: Vec<Entity>) -> Self {
        Inventory {
            items,
            max_weight: 400,
            current_weight: 0,
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
            println!("[{}]", item_components.get(*item).unwrap().name);
        }
    }

    pub fn equip(&mut self, equipment_slot: &mut Slot, inventory_position: usize) {
        equipment_slot.equip(self.items[inventory_position]);
    }
}

pub struct Attribute {
    base: i32
}

pub struct Attributes {
    pub health: Attribute,
    pub defence: Attribute,
}

impl Attributes {
    pub fn new(health_base: i32, defence_base: i32) -> Self {
        Attributes {
            health: Attribute { base: health_base },
            defence: Attribute { base: defence_base }
        }
    }
}