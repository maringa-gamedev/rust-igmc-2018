use amethyst::{
    core::{timing::Time, transform::Transform},
    ecs::prelude::{Entities, Join, Read, ReadStorage, System, WriteStorage},
    renderer::SpriteRender,
};
use crate::{constants::*, ecs::*};
use itertools::Itertools;
use log::*;
use nalgebra::{Isometry2, Vector2};
use ncollide2d::{
    query::{self, *},
    shape::*,
};

pub struct InteractSystem;

impl<'s> System<'s> for InteractSystem {
    type SystemData = (
        ReadStorage<'s, Player>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Hitbox>,
        ReadStorage<'s, Direction>,
        ReadStorage<'s, Interact>,
        WriteStorage<'s, SpriteRender>,
    );

    fn run(
        &mut self,
        (players, transforms, hitboxes, directions, interacts, mut sprites): Self::SystemData,
    ) {
        // Reset interaction highlight
        for interact in (&interacts).join() {
            let top = sprites.get_mut(interact.top).unwrap();
            top.sprite_number = interact.original;
        }

        for (_, transform, hitbox, direction) in
            (&players, &transforms, &hitboxes, &directions).join()
        {
            let check_pos = Isometry2::new(
                Vector2::new(
                    transform.translation.x + direction.current.make_interaction_offset_x(),
                    transform.translation.y + direction.current.make_interaction_offset_y(),
                ),
                nalgebra::zero(),
            );
            let check_shape = Cuboid::new(Vector2::new(
                hitbox.shape.half_extents().x * 0.9,
                hitbox.shape.half_extents().y * 0.9,
            ));
            let dir = direction.current;

            for (transform, hitbox, interact) in (&transforms, &hitboxes, &interacts)
                .join()
                .sorted_by(|(at, _, _), (bt, _, _)| {
                    // TODO: Could be improved by also looking at previous direction to determine
                    // ordering.
                    match dir {
                        Cardinal::North => bt.translation.y.partial_cmp(&at.translation.y),
                        Cardinal::NorthWest => bt.translation.y.partial_cmp(&at.translation.y),
                        Cardinal::NorthEast => bt.translation.y.partial_cmp(&at.translation.y),

                        Cardinal::South => at.translation.y.partial_cmp(&bt.translation.y),
                        Cardinal::SouthWest => at.translation.y.partial_cmp(&bt.translation.y),
                        Cardinal::SouthEast => at.translation.y.partial_cmp(&bt.translation.y),

                        Cardinal::West => at.translation.x.partial_cmp(&bt.translation.x),
                        Cardinal::East => bt.translation.x.partial_cmp(&at.translation.x),
                    }
                    .unwrap()
                }) {
                let interact_pos = Isometry2::new(
                    Vector2::new(
                        transform.translation.x + hitbox.offset.x,
                        transform.translation.y + hitbox.offset.y,
                    ),
                    nalgebra::zero(),
                );
                let interact_shape = &hitbox.shape;
                let top = sprites.get_mut(interact.top).unwrap();

                if let Proximity::Intersecting =
                    query::proximity(&check_pos, &check_shape, &interact_pos, interact_shape, 1.0)
                {
                    top.sprite_number = interact.highlight;
                    info!("CAN INTERACT!");
                    break;
                }
            }
        }
    }
}
