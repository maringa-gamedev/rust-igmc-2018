use amethyst::ecs::prelude::*;
use crate::constants::*;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Cardinal {
    North,     // Up
    NorthWest, // Up Left
    NorthEast, // Up Right

    South,     // Down
    SouthWest, // Down Left
    SouthEast, // Down Right

    West, // Left
    East, // Right
}

pub struct Direction {
    pub current: Cardinal,
    pub previous: Option<Cardinal>,
    pub north: usize,
    pub north_west: usize,
    pub north_east: usize,
    pub south: usize,
    pub south_west: usize,
    pub south_east: usize,
    pub west: usize,
    pub east: usize,
}

impl Cardinal {
    pub fn get_x(&self) -> f32 {
        match self {
            Cardinal::West => -1.0,
            Cardinal::NorthWest => -0.7071,
            Cardinal::SouthWest => -0.7071,

            Cardinal::East => 1.0,
            Cardinal::NorthEast => 0.7071,
            Cardinal::SouthEast => 0.7071,

            Cardinal::North => 0.0,
            Cardinal::South => 0.0,
        }
    }

    pub fn get_y(&self) -> f32 {
        match self {
            Cardinal::North => 1.0,
            Cardinal::NorthWest => 0.7071,
            Cardinal::NorthEast => 0.7071,

            Cardinal::South => -1.0,
            Cardinal::SouthWest => -0.7071,
            Cardinal::SouthEast => -0.7071,

            Cardinal::West => 0.0,
            Cardinal::East => 0.0,
        }
    }

    pub fn make_interaction_offset_x(&self) -> f32 {
        match self {
            Cardinal::North => 0.0,
            Cardinal::NorthWest => -BASE / 2.0,
            Cardinal::NorthEast => BASE / 2.0,

            Cardinal::South => 0.0,
            Cardinal::SouthWest => -BASE / 2.0,
            Cardinal::SouthEast => BASE / 2.0,

            Cardinal::West => -BASE / 2.0,
            Cardinal::East => BASE / 2.0,
        }
    }

    pub fn make_interaction_offset_y(&self) -> f32 {
        match self {
            Cardinal::North => BASE / 2.0,
            Cardinal::NorthWest => BASE / 2.0,
            Cardinal::NorthEast => BASE / 2.0,

            Cardinal::South => -BASE / 2.0,
            Cardinal::SouthWest => -BASE / 2.0,
            Cardinal::SouthEast => -BASE / 2.0,

            Cardinal::West => 0.0,
            Cardinal::East => 0.0,
        }
    }
}

impl Component for Direction {
    type Storage = DenseVecStorage<Self>;
}
