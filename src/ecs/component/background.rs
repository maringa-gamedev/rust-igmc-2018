use amethyst::{core::cgmath::*, ecs::prelude::*};

pub struct Background {
    pub from: Vector2<f32>,
    pub to: Vector2<f32>,
    pub timer: f32,
}

impl Component for Background {
    type Storage = DenseVecStorage<Self>;
}
