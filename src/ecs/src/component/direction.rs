use amethyst::ecs::prelude::*;
use log::*;
use nk_data::*;
use serde_derive::*;

pub struct Direction {
    pub current: Cardinal,
    pub previous: Option<Cardinal>,
    pub current_anim: String,
}

impl Component for Direction {
    type Storage = DenseVecStorage<Self>;
}
