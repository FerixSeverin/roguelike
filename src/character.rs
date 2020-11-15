pub struct Player;

pub struct Evil;

pub struct Attribute {
    base: i32
}

pub struct Attributes {
    pub health: Attribute,
}

impl Attributes {
    pub fn new(health_base: i32) -> Self {
        Attributes {
            health: Attribute { base: health_base }
        }
    }
}