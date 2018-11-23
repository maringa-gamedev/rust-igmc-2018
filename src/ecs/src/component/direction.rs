use amethyst::ecs::prelude::*;
use nk_data::*;

pub struct Direction {
    pub current: Cardinal,
    pub previous: Option<Cardinal>,
    pub current_anim: String,
}

impl Component for Direction {
    type Storage = DenseVecStorage<Self>;
}
