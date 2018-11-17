mod animation;
mod common;
mod constants;
mod effect;
mod flavor;
mod gameplay;
mod hissatsu;
mod house;
mod map;
mod order;
mod player;
mod preparation;
mod team;
mod texture;
mod topping;

mod def {
    #[derive(Debug, Default)]
    pub struct Definitions {
        flavors: Vec<FlavorDef>,
        preparations: Vec<PreparationDef>,
        toppings: Vec<ToppingDef>,
        hissatsu: Vec<HissatsuDef>,
        houses: Vec<HouseDef>,
        teams: Vec<TeamDef>,
        players: Vec<PlayerDef>,
    }

    impl Definitions {
        pub fn new(f: Vec<FlavorDef>, p: Vec<PreparationDef>, t: Vec<ToppingDef>) -> Self {
            Definitions {
                flavors: f,
                preparations: p,
                toppings: t,
                hissatsu: Vec::new(),
                houses: Vec::new(),
                teams: Vec::new(),
                players: Vec::new(),
            }
        }

        pub fn flavors(&self) -> std::slice::Iter<FlavorDef> {
            self.flavors.iter()
        }

        pub fn preparations(&self) -> std::slice::Iter<PreparationDef> {
            self.preparations.iter()
        }

        pub fn toppings(&self) -> std::slice::Iter<ToppingDef> {
            self.toppings.iter()
        }
    }

    pub use super::{
        flavor::FlavorDef, hissatsu::HissatsuDef, house::HouseDef, player::PlayerDef,
        preparation::PreparationDef, team::TeamDef, topping::ToppingDef,
    };
}

pub use self::{
    animation::*, common::*, constants::*, def::*, effect::*, flavor::*, gameplay::*, hissatsu::*,
    house::*, map::*, order::*, player::*, preparation::*, team::*, texture::*, topping::*,
};
