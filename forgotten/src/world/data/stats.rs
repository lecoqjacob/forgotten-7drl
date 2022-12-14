use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Armour {
    pub value: u32,
}

impl Armour {
    pub const fn new(value: u32) -> Self {
        Self { value }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct HitPoints {
    pub current: u32,
    pub max: u32,
}

impl HitPoints {
    pub const fn new_full(max: u32) -> Self {
        Self { current: max, max }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Stunned {
    pub turns: u32,
}
