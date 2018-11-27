use super::{common::*, effect::*};
use serde_derive::*;

pub type Position = (f32, f32);

#[derive(Debug, Serialize, Deserialize)]
pub struct PreparationFlavorOffsets {
    pub one: [Position; 1],
    pub two: [Position; 2],
    pub three: [Position; 3],
    pub four: [Position; 4],
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreparationDef {
    pub index: PreparationIndex,
    pub key: String,
    pub score_multiplier: f32,
    pub melt_multiplier: Option<f32>,
    pub effect_area_multiplier: f32,
    pub max_scoops: usize,
    pub effect: Vec<EffectDefinition>,
    pub score_multiplier_condition: Vec<EffectCondition>,
    pub takes_topping: bool,
    pub offsets: PreparationFlavorOffsets,
}
