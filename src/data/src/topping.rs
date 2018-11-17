use super::{common::*, effect::*};
use serde_derive::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct ToppingDef {
    pub index: ToppingIndex,
    pub key: String,
    pub worth: f32,
    pub effect: Vec<EffectDefinition>,
}
