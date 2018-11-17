use amethyst::{core::cgmath::Vector2, ecs::prelude::*};
use crate::component::*;
use nk_data::*;

pub struct Input {
    pub wants_to_move: Option<Cardinal>,
    pub last_moved_direction: Option<Cardinal>,
    pub wants_to_interact: bool,
}

impl Input {
    pub fn new() -> Self {
        Input {
            wants_to_move: None,
            last_moved_direction: None,
            wants_to_interact: false,
        }
    }
}

impl Component for Input {
    type Storage = DenseVecStorage<Self>;
}
