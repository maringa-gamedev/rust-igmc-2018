use super::{common::*, effect::*};
use serde_derive::*;

pub type Position = (f32, f32);

#[derive(Debug, Serialize, Deserialize)]
pub struct ToppingFlavorOffsets {
    pub one: Position,
    pub two: Position,
    pub three: Position,
    pub four: Position,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ToppingDef {
    pub index: ToppingIndex,
    pub key: String,
    pub worth: f32,
    pub effect: Vec<EffectDefinition>,
    pub offsets: ToppingFlavorOffsets,
}
