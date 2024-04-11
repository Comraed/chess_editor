use std::ops::{Add, Sub};

use bevy::{ecs::component::Component, reflect::Reflect};

#[derive(Component, Debug, PartialEq, Eq, Hash, Reflect)]
pub struct PieceCoord{
    x: i8,
    y: i8,
}

impl PieceCoord {
    pub fn new(x: i8, y: i8) -> Self{
        Self{x, y}
    }
}

impl Add for PieceCoord{
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for PieceCoord{
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}