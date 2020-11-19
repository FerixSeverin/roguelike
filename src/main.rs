mod config;
mod turn;
mod world;
mod mobility;
mod character;
mod item;

use bevy::prelude::*;
use bevy::render::pass::ClearColor;
use std::collections::HashMap;

//Events

//Globals

struct Tile {
    pub main: Option<Entity>,
    pub temperature: i16,
    pub air: Option<Entity>,
    pub ground: Option<Entity>,
    pub base: Option<Entity>,
}

type World<T> = HashMap<(i32, i32), T>;

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

struct Size {
    width: f32,
    height: f32,
}

impl Size {
    pub fn square(x: f32) -> Self {
        Self {
            width: x,
            height: x,
        }
    }
}

//Materials

struct Materials {
    player: Handle<ColorMaterial>,
    wall: Handle<ColorMaterial>,
    pit: Handle<ColorMaterial>,
    evil: Handle<ColorMaterial>,
}

//Work in progress components, will be moved later to separate files

struct Pit;

struct Model {
    name: String,
    serial_number: String,
}

// Systems

fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn((turn::Counter {}, turn::InQueue));
    commands.insert_resource(turn::Queue::new(commands.current_entity().unwrap()));
    commands.insert_resource(World::<Tile>::new());

    commands.spawn(Camera2dComponents::default());
    commands.insert_resource(Materials {
        player: materials.add(Color::rgb_u8(0, 163, 204).into()),
        wall: materials.add(Color::rgb_u8(217, 217, 217).into()),
        pit: materials.add(Color::rgb_u8(64, 64, 64).into()),
        evil: materials.add(Color::rgb_u8(237, 76, 47).into()),
    });
}

fn world_setup(
    mut commands: Commands,
    mut turn_queue: ResMut<turn::Queue>,
    mut world: ResMut<World<Tile>>,
    materials: Res<Materials>,
) {
    commands.spawn((item::Item::new("Knife", "Desc", 15), item::Melee::new(5), item::Equippable::new(item::EqiuppedInto::Weapon), item::Damage { base: 5 }));
    let knife_id: Entity = commands.current_entity().unwrap();

    commands.spawn((item::Item::new("Health Potion", "Desc", 5), item::Consumable));
    let potion_id: Entity = commands.current_entity().unwrap();

    let w: world::World = world::load_world(format!("{}/world.ron", env!("CARGO_MANIFEST_DIR")));
    let mut y: isize = (w.get().len() - 1) as isize;
    for row in w.get() {
        for (x, tile) in row.chars().enumerate() {
            //let mut sprite_component: SpriteComponents;
            match tile {
                '.' => {
                    world.insert((x as i32, y as i32), Tile::empty());
                }
                '#' => {
                    commands
                        .spawn(SpriteComponents {
                            material: materials.wall.clone(),
                            sprite: Sprite::new(Vec2::new(20.0, 20.0)),
                            ..Default::default()
                        })
                        .with(mobility::Blocking)
                        .with(mobility::Position {
                            x: x as i32,
                            y: y as i32,
                        });
                    world.insert((x as i32, y as i32), Tile::main(commands.current_entity().unwrap()));
                }
                '@' => {
                    commands
                        .spawn(SpriteComponents {
                            material: materials.player.clone(),
                            sprite: Sprite::new(Vec2::new(20.0, 20.0)),
                            ..Default::default()
                        })
                        .with(character::Player)
                        .with(mobility::Position {
                            x: x as i32,
                            y: y as i32,
                        })
                        .with(mobility::Walkable { step_size: 1 })
                        .with(character::Attributes::new(20, 5))
                        .with(turn::InQueue)
                        .with(turn::Head)
                        .with(character::Inventory::starting_inventory(vec![knife_id, potion_id]))
                        .with(character::Equipment::naked());
                    turn_queue.add_zero(commands.current_entity().unwrap());
                    world.insert((x as i32, y as i32), Tile::main(commands.current_entity().unwrap()));
                }
                'P' => {
                    commands
                        .spawn(SpriteComponents {
                            material: materials.pit.clone(),
                            sprite: Sprite::new(Vec2::new(15.0, 15.0)),
                            ..Default::default()
                        })
                        .with(Pit)
                        .with(mobility::Position {
                            x: x as i32,
                            y: y as i32,
                        });
                    world.insert((x as i32, y as i32), Tile::base(commands.current_entity().unwrap()));
                }
                'E' => {
                    commands
                        .spawn(SpriteComponents {
                            material: materials.evil.clone(),
                            sprite: Sprite::new(Vec2::new(18.0, 18.0)),
                            ..Default::default()
                        })
                        .with(character::Evil)
                        .with(mobility::Position {
                            x: x as i32,
                            y: y as i32,
                        })
                        .with(mobility::Walkable { step_size: 1 })
                        .with(character::Attributes::new(5, 0))
                        .with(Model {
                            name: "E1-L".to_string(),
                            serial_number: "XXXXXX-XX".to_string(),
                        })
                        .with(turn::InQueue);
                    //turn_queue.add_zero(commands.current_entity().unwrap());
                    world.insert((x as i32, y as i32), Tile::main(commands.current_entity().unwrap()));
                }
                _ => continue,
            };
        }
        y -= 1;
    }
}

fn player_attack(
    keyboard_input: Res<Input<KeyCode>>,
    mut events: ResMut<Events<turn::Done>>,
    mut turn_queue: ResMut<turn::Queue>,
    mut players: Query<(
        Entity,
        &mut character::Player,
        &mut mobility::Position,
        &mut turn::Head,
        &character::Equipment,
    )>,
    mut targets: Query<(
        Entity,
        &character::Evil,
        &mut character::Attributes,
        &mobility::Position,
    )>,
    weapons: Query<(
        &item::Item,
        &item::Equippable,
        &item::Damage,
    )>,
) {
    let mut attempted_to_attack = false;
    let mut attack_direction = (0, 0);
    if keyboard_input.just_pressed(KeyCode::W) {
        attack_direction = (0, 1);
        attempted_to_attack = true;
    } else if keyboard_input.just_pressed(KeyCode::D) {
        attack_direction = (1, 0);
        attempted_to_attack = true;
    } else if keyboard_input.just_pressed(KeyCode::S) {
        attack_direction = (0, -1);
        attempted_to_attack = true;
    } else if keyboard_input.just_pressed(KeyCode::A) {
        attack_direction = (-1, 0);
        attempted_to_attack = true;
    }

    if attempted_to_attack {
        for (_entity, _player, player_position, _head, _equipment) in players.iter_mut() {
            let attack_position = mobility::Position { x: player_position.x + attack_direction.0, y: player_position.y + attack_direction.1 };
            for (_entity, _evil, mut attributes, target_position) in targets.iter_mut() {
                if target_position.eq(&attack_position) {
                    attributes.health.modifier -= _equipment.get_weapon_damage(&weapons);
                    println!("Attacked! Targets current health is: {}", attributes.health.current());
                    turn_queue.head_makes_action(100);
                    events.send(turn::Done);
                }
            }
        }
    }
}

fn player_movement(
    //map: Res<Array2<Entity>>,
    mut commands: Commands,
    mut turn_queue: ResMut<turn::Queue>,
    mut world: ResMut<World<Tile>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut events: ResMut<Events<turn::Done>>,
    mut players: Query<(
        Entity,
        &mut character::Player,
        &mobility::Walkable,
        &mut mobility::Position,
        &mut turn::Head,
    )>,
    walls: Query<(Entity, &mobility::Blocking, &mobility::Position)>,
) {
    for (player_entity, mut _player, walkable, mut position, _head) in players.iter_mut() {
        let mut attempted_to_walk = false;
        let mut blocked = false;

        let step_size = walkable.step_size;
        let mut position_change = (0, 0);

        if keyboard_input.just_pressed(KeyCode::Up) {
            position_change = (0, step_size);
            attempted_to_walk = true;
        } else if keyboard_input.just_pressed(KeyCode::Right) {
            position_change = (step_size, 0);
            attempted_to_walk = true;
        } else if keyboard_input.just_pressed(KeyCode::Down) {
            position_change = (0, -step_size);
            attempted_to_walk = true;
        } else if keyboard_input.just_pressed(KeyCode::Left) {
            position_change = (-step_size, 0);
            attempted_to_walk = true;
        }

        if attempted_to_walk {
            match world.get(&(position.x + position_change.0, position.y + position_change.1)) {
                Some(tile) => match tile.main {
                    Some(_main_entity) => {
                        blocked = true;
                    },
                    None => ()
                },
                None => ()
            }

            if !blocked {
                position.translate(position_change.0, position_change.1);
                println!("Walked!");
                if !turn_queue.head_makes_action(100) {
                    commands.remove_one::<turn::Head>(player_entity);
                    events.send(turn::Done);
                }
            }
        }
    }
}

fn position_translation(mut q: Query<(&mobility::Position, &mut Transform)>) {
    fn convert(p: i32, position_multiplier: i32) -> i32 {
        p * position_multiplier
    }
    for (pos, mut transform) in q.iter_mut() {
        transform.translation =
            Vec3::new(convert(pos.x, 20) as f32, convert(pos.y, 20) as f32, 0.0);
    }
}

fn pit_mechanic(
    mut commands: Commands,
    pits: Query<(Entity, &Pit, &mobility::Position)>,
    mut walkable_entities: Query<(Entity, &mut mobility::Walkable, &mobility::Position)>,
) {
    for (_pit_entity, _pit, pit_positon) in pits.iter() {
        for (walkable_entity, _walkable, walking_position) in walkable_entities.iter_mut() {
            if pit_positon.eq(walking_position) {
                println!("Fell into pit");
                commands.remove_one::<mobility::Walkable>(walkable_entity);
            }
        }
    }
}

fn get_legs(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    mut players: Query<(Entity, &character::Player)>,
) {
    for (player_entity, _player) in players.iter_mut() {
        if keyboard_input.just_pressed(KeyCode::X) {
            println!("Got legs");
            commands.insert_one(player_entity, mobility::Walkable { step_size: 1 });
        }
    }
}

fn turn_management(
    mut commands: Commands,
    mut turn_queue: ResMut<turn::Queue>,
    mut event_reader: Local<EventReader<turn::Done>>,
    events: Res<Events<turn::Done>>,
    entities: Query<(Entity, &turn::InQueue)>,
) {
    for _event in event_reader.iter(&events) {
        println!("Next Turn");
        let head_entity = turn_queue.peek().entity;
        let head = entities.get(head_entity).unwrap().0;
        commands.insert_one(head, turn::Head);
    }
}

fn turn_tick(
    mut commands: Commands,
    mut turn_queue: ResMut<turn::Queue>,
    mut events: ResMut<Events<turn::Done>>,
    mut turns: Query<(Entity, &turn::Counter, &turn::InQueue, &turn::Head)>,
) {
    for (entity, _counter, _in_queue, _head) in turns.iter_mut() {
        //println!("Turn ticked!");
        turn_queue.head_makes_action(100);
        commands.remove_one::<turn::Head>(entity);
        events.send(turn::Done);
    }
}

fn inventory_management(
    keyboard_input: Res<Input<KeyCode>>,
    mut players: Query<(Entity, &character::Player, &mut character::Inventory, &mut character::Equipment)>,
    items: Query<&item::Item>,
    equippable_items: Query<(&item::Item, &item::Equippable)>,
) {
    if keyboard_input.just_pressed(KeyCode::I) {
        for (_entity, _player, _inventory, _equipment) in players.iter_mut() {
            _inventory.look(&items);
        }
    }

    if keyboard_input.just_pressed(KeyCode::K) {
        for (_entity, _player, mut _inventory, mut _equipment) in players.iter_mut() {
            _inventory.equip(&mut _equipment.weapon, 0, &equippable_items);
        }
    }
}

fn equipment_management(
    keyboard_input: Res<Input<KeyCode>>,
    players_with_equipment: Query<(Entity, &character::Player, &character::Equipment)>,
    equippable_items: Query<(&item::Item, &item::Equippable)>,
) {
    for (_entity, _player, _equipment) in players_with_equipment.iter() {
        if keyboard_input.just_pressed(KeyCode::O) {
            _equipment.look(&equippable_items);
        }
    }
}

fn death(
    mut commands: Commands,
    mut entities: Query<(Entity, &mut character::Attributes)>,
) {
    for (entity, attributes) in entities.iter_mut() {
        if attributes.health.current() <= 0 {
            println!("Entity died!");
            commands.despawn(entity);
        }
    }
}

fn main() {
    let loaded_config = config::load_config(format!("{}/config.ron", env!("CARGO_MANIFEST_DIR")));
    let world_stage = "world_stage";

    App::build()
        .add_resource(ClearColor(Color::rgb(0.01, 0.01, 0.12)))
        .add_resource(WindowDescriptor {
            width: loaded_config.window_width(),
            height: loaded_config.window_height(),
            title: loaded_config.window_title().to_string(),
            ..Default::default()
        })
        /*.add_resource(Array2::<Entity>(HashMap::new()))*/
        .add_startup_system(setup.system())
        .add_startup_stage(world_stage)
        .add_startup_system_to_stage(world_stage, world_setup.system())
        .add_event::<turn::Done>()
        .add_system(player_movement.system())
        .add_system(position_translation.system())
        .add_system(pit_mechanic.system())
        .add_system(get_legs.system())
        .add_system(turn_management.system())
        .add_system(turn_tick.system())
        .add_system(inventory_management.system())
        .add_system(equipment_management.system())
        .add_system(player_attack.system())
        .add_system(death.system())
        .add_plugins(DefaultPlugins)
        .run();
}
