use amethyst::ecs::prelude::*;
use nk_data::*;

pub struct Input {
    pub wants_to_move: Option<Cardinal>,
    pub last_moved_direction: Option<Cardinal>,
    pub wants_to_interact: bool,
    pub wants_north: bool,
    pub wants_south: bool,
    pub wants_west: bool,
    pub wants_east: bool,
}

impl Input {
    pub fn new() -> Self {
        Input {
            wants_to_move: None,
            last_moved_direction: None,
            wants_to_interact: false,
            wants_north: false,
            wants_south: false,
            wants_west: false,
            wants_east: false,
        }
    }
}

impl Component for Input {
    type Storage = DenseVecStorage<Self>;
}
