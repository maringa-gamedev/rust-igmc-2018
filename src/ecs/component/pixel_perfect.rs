use amethyst::{core::cgmath::*, ecs::prelude::*};

pub struct PixelPerfect {
    pub position: Vector2<f32>,
}

impl Component for PixelPerfect {
    type Storage = DenseVecStorage<Self>;
}
