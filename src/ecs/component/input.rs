use amethyst::{core::cgmath::Vector2, ecs::prelude::*};
use crate::ecs::*;

pub struct Input {
    pub wants_to_move: Option<Cardinal>,
    pub last_moved_direction: Option<Cardinal>,
}

impl Input {
    pub fn new(initial_direction: Vector2<f32>) -> Self {
        Input {
            wants_to_move: None,
            last_moved_direction: None,
        }
    }
}

impl Component for Input {
    type Storage = DenseVecStorage<Self>;
}
