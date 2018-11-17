use amethyst::ecs::prelude::*;

pub struct Interact {
    pub highlighted_by: Option<usize>,
    pub top: Entity,
}

impl Component for Interact {
    type Storage = DenseVecStorage<Self>;
}
