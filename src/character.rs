use crate::item;
use bevy::prelude::{Entity};
use bevy::ecs::Query;

#[derive(PartialEq)]
pub enum MovementBehaviour {
    StandStill,
    FollowPlayer,
    Wander,
    Stunned,
    PlayerControlled,
}

pub enum AttackBehaviour {
    CantAttack,
    AttackFriendly,
    AttackPlayer,
    PlayerControlled,
}

pub struct Archetype {
    pub name: String,
    pub movement_behaviour: MovementBehaviour,
    pub attack_behaviour: AttackBehaviour,
}

pub struct AI {
    pub archetype: Archetype,
}

impl AI {
    pub fn new(archetype: Archetype) -> Self {
        Self {
            archetype,
        }
    }

    pub fn hostile() -> Self {
        Self {
            archetype: Archetype {
                name: "Evil".to_string(),
                movement_behaviour: MovementBehaviour::Wander,
                attack_behaviour: AttackBehaviour::AttackPlayer,
            },
        }
    }

    pub fn player() -> Self {
        Self {
            archetype: Archetype {
                name: "Player".to_string(),
                movement_behaviour: MovementBehaviour::PlayerControlled,
                attack_behaviour: AttackBehaviour::PlayerControlled,
            },
        }
    }

    pub fn friendly() -> Self {
        Self {
            archetype: Archetype {
                name: "Friendly".to_string(),
                movement_behaviour: MovementBehaviour::StandStill,
                attack_behaviour: AttackBehaviour::CantAttack,
            },
        }
    }

    pub fn movement(&self) -> &MovementBehaviour {
        &self.archetype.movement_behaviour
    }

    pub fn attack(&self) -> &AttackBehaviour {
        &self.archetype.attack_behaviour
    }
}

pub struct Slot {
    entity: Option<Entity>,
    equipped_into: item::EqiuppedInto,
}

impl Slot {
    pub fn nothing(equipped_into: item::EqiuppedInto) -> Self {
        Slot {
            entity: None,
            equipped_into,
        }
    }

    pub fn equip(&mut self, entity: Entity, equipped_into: &item::EqiuppedInto) -> bool {
        if self.equipped_into.eq(equipped_into) {
            self.entity = Option::from(entity);
            true
        } else {
            println!("Item does not fit the slot!");
            false
        }

    }
}

pub struct Equipment {
    pub weapon: Slot,
    pub armor: Slot,
}

impl Equipment {
    pub fn naked() -> Self {
        Equipment {
            weapon: Slot::nothing(item::EqiuppedInto::Weapon),
            armor: Slot::nothing(item::EqiuppedInto::Armor),
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

    pub fn get_weapon_damage(&self, weapons: &Query<(&item::Item, &item::Equippable, &item::Damage)>) -> i32 {
        if self.weapon.entity.is_some() {
            weapons.get_component::<item::Damage>(self.weapon.entity.unwrap()).unwrap().base
        } else {
            println!("No Weapon is equipped!");
            0
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

    pub fn look(&self, item_components: &Query<&item::Item>) {
        if self.items.is_empty() {
            println!("Inventory is empty");
            return;
        }

        for item in &self.items {
            println!("[{}]", item_components.get(*item).unwrap().name);
        }
    }

    pub fn equip(&mut self, equipment_slot: &mut Slot, inventory_position: usize, equippable_items: &Query<(&item::Item, &item::Equippable)>) {
        if self.items.is_empty() {
            println!("Inventory is empty");
            return;
        }

        let equipment: Entity = self.items[inventory_position];
        if equippable_items.get(equipment).is_ok() {
            let equipped_into: &item::EqiuppedInto = &equippable_items.get(equipment).unwrap().1.equipped_into;
            if equipment_slot.equip(equipment, equipped_into) {
                self.items.remove(inventory_position);
            }
        } else {
            println!("Selected item can not be equipped");
        }

    }
}

pub struct Attribute {
    pub base: i32,
    pub modifier: i32,
}

impl Attribute {
    pub fn current(&self) -> i32 {
        self.base + self.modifier
    }
}

pub struct Attributes {
    pub health: Attribute,
    pub defence: Attribute,
}

impl Attributes {
    pub fn new(health_base: i32, defence_base: i32) -> Self {
        Attributes {
            health: Attribute { base: health_base, modifier: 0 },
            defence: Attribute { base: defence_base, modifier: 0 }
        }
    }
}