use super::common::*;
use serde_derive::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct HissatsuDef {
    pub index: HissatsuIndex,
    pub key: String,
}
