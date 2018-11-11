use amethyst::ecs::prelude::*;
use ncollide2d::shape::*;

pub struct Effect {
    pub shape: Ball<f32>,
}

impl Component for Effect {
    type Storage = DenseVecStorage<Self>;
}
