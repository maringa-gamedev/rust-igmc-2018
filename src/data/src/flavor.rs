use super::{common::*, effect::*};
use serde_derive::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct FlavorDef {
    pub index: FlavorIndex,
    pub key: String,
    pub class: FlavorClass,
    pub base_worth: f32,
    pub effect: Vec<EffectDefinition>,
    pub condition: Vec<EffectCondition>,
}
