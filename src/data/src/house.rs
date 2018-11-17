use super::common::*;
use serde_derive::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct HouseDef {
    pub index: HouseIndex,
    pub key: String,
}
