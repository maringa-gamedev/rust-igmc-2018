use super::{common::*, effect::*};
use serde_derive::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct PreparationDef {
    pub index: ToppingIndex,
    pub key: String,
    pub score_multiplier: f32,
    pub melt_multiplier: Option<f32>,
    pub effect_area_multiplier: f32,
    pub max_scoops: usize,
    pub effect: Vec<EffectDefinition>,
    pub score_multiplier_condition: Vec<EffectCondition>,
    pub takes_topping: bool,
}
