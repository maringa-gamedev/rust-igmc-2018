use amethyst::{core::cgmath::Vector2, ecs::prelude::*};

pub enum Step {
    Absolute,
    Relative,
}

pub struct Velocity {
    pub velocity: Vector2<f32>,
    pub current: Vector2<f32>,
    pub step: Step,
}

impl Velocity {
    pub fn new(xy: f32) -> Self {
        Velocity {
            velocity: Vector2::new(xy, xy),
            current: Vector2::new(0.0, 0.0),
            step: Step::Absolute,
        }
    }
}

impl Component for Velocity {
    type Storage = DenseVecStorage<Self>;
}
