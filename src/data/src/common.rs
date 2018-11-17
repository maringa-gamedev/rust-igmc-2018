use super::constants::*;
use serde_derive::*;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum FlavorClass {
    Classic,
    Sherbert,
    Special,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct FlavorIndex(pub usize);
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ToppingIndex(pub usize);
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct PreparationIndex(pub usize);
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct HissatsuIndex(pub usize);
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct TeamIndex(pub usize);
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct HouseIndex(pub usize);
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct PlayerIndex(pub usize);

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
