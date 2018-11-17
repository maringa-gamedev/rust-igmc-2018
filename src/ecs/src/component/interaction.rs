use amethyst::ecs::prelude::*;

pub struct Interaction {
    pub margin: f32,
}

impl Component for Interaction {
    type Storage = DenseVecStorage<Self>;
}
