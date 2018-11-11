use amethyst::ecs::prelude::*;

pub struct Solid;

impl Component for Solid {
    type Storage = DenseVecStorage<Self>;
}
