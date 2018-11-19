use amethyst::ecs::prelude::*;
use either::*;
use nk_data::*;

pub struct Player {
    pub gamepad_index: usize,
    pub layer: f32,
    pub inventory: Option<Either<FlavorIndex, Order>>,
    pub invert_x_axis: bool,
}

impl Player {
    pub fn new(gamepad_index: usize, layer: f32, invert_x_axis: bool) -> Self {
        Player {
            gamepad_index,
            layer,
            inventory: None,
            invert_x_axis,
        }
    }
}

impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}
