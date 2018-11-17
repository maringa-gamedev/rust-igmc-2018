use amethyst::ecs::prelude::*;
use either::*;
use nk_data::*;

pub struct Player {
    pub gamepad_index: usize,
    pub layer: f32,
    pub inventory: Option<Either<FlavorIndex, Order>>,
}

impl Player {
    pub fn new(gamepad_index: usize, layer: f32) -> Self {
        Player {
            gamepad_index,
            layer,
            inventory: None,
        }
    }
}

impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}
