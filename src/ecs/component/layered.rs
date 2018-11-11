use amethyst::ecs::*;

pub struct Layered;

impl Component for Layered {
    type Storage = DenseVecStorage<Self>;
}
