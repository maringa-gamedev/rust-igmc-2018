use amethyst::{core::cgmath::*, ecs::prelude::*, renderer::SpriteRender};
use crate::{ecs::*, util::*};
use log::*;

pub struct DirectionSystem;

impl<'s> System<'s> for DirectionSystem {
    type SystemData = (
        ReadStorage<'s, Input>,
        WriteStorage<'s, Direction>,
        WriteStorage<'s, SpriteRender>,
    );

    fn run(&mut self, (inputs, mut directions, mut sprites): Self::SystemData) {
        for (input, direction, sprite) in (&inputs, &mut directions, &mut sprites).join() {
            sprite.sprite_number = match direction.current {
                Cardinal::North => direction.north,
                Cardinal::NorthWest => direction.north_west,
                Cardinal::NorthEast => direction.north_east,

                Cardinal::South => direction.south,
                Cardinal::SouthWest => direction.south_west,
                Cardinal::SouthEast => direction.south_east,

                Cardinal::West => direction.west,
                Cardinal::East => direction.east,
            };
        }
    }
}
