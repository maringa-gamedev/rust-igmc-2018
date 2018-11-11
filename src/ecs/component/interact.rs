use amethyst::ecs::prelude::*;

pub struct Interact {
    pub top: Entity,
    pub highlight: usize,
    pub original: usize,
}

impl Component for Interact {
    type Storage = DenseVecStorage<Self>;
}
