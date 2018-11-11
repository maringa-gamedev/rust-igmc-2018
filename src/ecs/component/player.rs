use amethyst::ecs::prelude::*;

pub struct Player {
    pub gamepad_index: usize,
    pub layer: f32,
}

impl Player {
    pub fn new(gamepad_index: usize, layer: f32) -> Self {
        Player {
            gamepad_index,
            layer,
        }
    }
}

impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}
