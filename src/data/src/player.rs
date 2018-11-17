use super::common::*;
use serde_derive::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayerDef {
    pub index: PlayerIndex,
    pub name: String,
    pub primary_flavor: FlavorIndex,
    pub secondary_flavors: Vec<FlavorIndex>,
    pub preparations: Vec<PreparationIndex>,
    pub toppings: Vec<ToppingIndex>,
    pub hissatsu: Vec<HissatsuIndex>,
    pub team: Option<TeamIndex>,
    pub house: Option<HouseIndex>,
}
