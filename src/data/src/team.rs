use super::common::*;
use serde_derive::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct TeamDef {
    pub index: TeamIndex,
    pub key: String,
}
