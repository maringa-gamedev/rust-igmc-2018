use super::{common::*, order::*};
use amethyst::ecs::prelude::*;

#[derive(Debug)]
pub struct Team {
    pub captain: Entity,
    pub server: Entity,
    pub scooper_one: Option<Entity>,
    pub scooper_two: Option<Entity>,
    pub flavors: Vec<FlavorIndex>,
    pub preparations: Vec<PreparationIndex>,
    pub toppings: Vec<ToppingIndex>,
    pub power_meter: f32,
    pub score: isize,
    pub orders: Vec<OrderDefinition>,
    pub parent: Entity,
}

#[derive(Debug, Default)]
pub struct Match {
    pub teams: Vec<Team>,
    pub flavors: Vec<FlavorIndex>,
    pub preparations: Vec<PreparationIndex>,
    pub toppings: Vec<ToppingIndex>,
    pub order_gen_timer: f32,
    pub timer: f32,
}
