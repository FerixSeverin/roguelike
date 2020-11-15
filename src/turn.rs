use bevy::prelude::Entity;

// Events

pub struct Done;

// Components

pub struct InQueue;

pub struct Head;

pub struct Counter;

// Logic

#[derive(Copy, Clone, Ord, Eq, PartialEq, PartialOrd)]
pub struct Event {
    time: u64,
    pub entity: Entity,
}

pub struct Queue {
    pub events: Vec<Event>,
    turn_entity: Entity,
}

impl Queue {
    pub fn new(turn_entity: Entity) -> Self {
        Queue {
            events: vec![Event {
                time: 0,
                entity: turn_entity,
            }],
            turn_entity,
        }
    }

    pub fn add(&mut self, entity: Entity, time: u64) {
        self.events.push(Event { time, entity });
    }

    pub fn add_zero(&mut self, entity: Entity) {
        self.events.push(Event { time: 0, entity });
    }

    pub fn peek(&mut self) -> Event {
        self.events[0]
    }

    pub fn head_makes_action(&mut self, time: u64) -> bool {
        let mut still_head: bool;
        self.events[0].time += time;
        if self.events[0].time <= self.events[1].time {
            still_head = true;
        } else {
            still_head = false;
            self.events.sort_by(|a, b| b.time.cmp(&a.time));
            self.events.reverse();
        }
        still_head
    }

    pub fn sort(&mut self) {
        self.events.sort_by(|a, b| b.time.cmp(&a.time));
        self.events.reverse();
    }

    pub fn print(&mut self) {
        for event in &self.events {
            println! {"{}, {}", event.entity.id().to_string(), event.time.to_string()};
        }
    }
}
