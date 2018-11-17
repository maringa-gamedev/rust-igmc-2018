use amethyst::{
    core::{timing::Time, transform::Transform},
    ecs::prelude::{Entities, Join, Read, ReadStorage, System, WriteStorage},
    renderer::SpriteRender,
};
use crate::component::*;
use either::*;
use itertools::Itertools;
use log::*;
use nalgebra::{Isometry2, Vector2};
use ncollide2d::{
    query::{self, *},
    shape::*,
};
use nk_data::*;

pub struct InteractSystem;

impl<'s> System<'s> for InteractSystem {
    type SystemData = (
        WriteStorage<'s, Player>,
        ReadStorage<'s, Input>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Hitbox>,
        ReadStorage<'s, Direction>,
        ReadStorage<'s, Interact>,
        WriteStorage<'s, Table>,
        WriteStorage<'s, SpriteRender>,
    );

    fn run(
        &mut self,
        (mut players, inputs, transforms, hitboxes, directions, interacts, mut tables, mut sprites): Self::SystemData,
    ) {
        // Reset interaction highlight
        for interact in (&interacts).join() {
            let top = sprites.get_mut(interact.top).unwrap();
            top.sprite_number = interact.original;
        }

        for (mut player, input, transform, hitbox, direction) in
            (&mut players, &inputs, &transforms, &hitboxes, &directions).join()
        {
            let check_pos = Isometry2::new(
                Vector2::new(
                    transform.translation.x + direction.current.make_interaction_offset_x(),
                    transform.translation.y + direction.current.make_interaction_offset_y(),
                ),
                nalgebra::zero(),
            );
            let check_shape = match &hitbox.shape {
                Either::Left(cuboid) => Cuboid::new(Vector2::new(
                    cuboid.half_extents().x * 0.9,
                    cuboid.half_extents().y * 0.9,
                )),
                Either::Right(ball) => {
                    Cuboid::new(Vector2::new(ball.radius() * 0.9, ball.radius() * 0.9))
                }
            };
            let dir = direction.current;

            for (transform, hitbox, interact) in (&transforms, &hitboxes, &interacts)
                .join()
                .sorted_by(|(at, _, _), (bt, _, _)| {
                    // TODO: Could be improved by also looking at previous direction to determine
                    // interaction.
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
                let interact_shape = match &hitbox.shape {
                    Either::Left(cuboid) => cuboid,
                    Either::Right(_) => panic!("DO NOT USE BALLS FOR INTERACTION COLLISION"),
                };
                let top = sprites.get_mut(interact.top).unwrap();

                if let Proximity::Intersecting =
                    query::proximity(&check_pos, &check_shape, &interact_pos, interact_shape, 1.0)
                {
                    top.sprite_number = interact.highlight;
                    info!("CAN INTERACT!");
                    if input.wants_to_interact {
                        if let Some(table) = tables.get_mut(interact.top) {
                            if let Some(flavor) = table.flavor() {
                                // FLAVOR TABLE
                                info!("TRYING TO SCOOP FLAVOR!");
                                if let Some(_item) = &player.inventory {
                                    info!("PLAYER CANNOT SCOOP AS INVENTORY IS FULL!");
                                } else {
                                    player.inventory = Some(Either::Left(flavor.clone()));
                                }
                            } else if let Some(preparation) = table.preparation() {
                                // PREP TABLE
                                info!("TRYING TO PREPARE ORDER!");
                                if let Some(item) = &player.inventory {
                                    // If player has an item in hand, check which.
                                    match item {
                                        Either::Left(flavor) => {
                                            // Insert scoops into order if possible.
                                            if table.has_order() {
                                                let mut order = table.extract_order();
                                                if order.ingredient_count() > 3 {
                                                    info!(
                                                        "ORDER IS FULL, CANNOT INSERT MORE FLAVORS"
                                                    );
                                                } else {
                                                    order.insert_flavor(flavor.clone());
                                                    player.inventory = None;
                                                }
                                                table.insert_order(order);
                                            } else {
                                                table.insert_order(Order::new(
                                                    preparation.clone(),
                                                    flavor.clone(),
                                                ));
                                                player.inventory = None;
                                            }
                                        }
                                        Either::Right(order) => {
                                            // Insert order back in if not completed and table is
                                            // empty.
                                            if order.is_completed() {
                                                info!("ORDER IS COMPLETED, CANNOT GO BACK IN!");
                                            } else {
                                                if table.has_order() {
                                                    info!("TABLE IS FULL, MAYBE SWAP SEMANTICS?");
                                                } else {
                                                    table.insert_order(order.clone());
                                                    player.inventory = None;
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    info!("PLAYER HAS NOTHING IN INVENTORY!");
                                    let order = table.extract_order();
                                    player.inventory = Some(Either::Right(order));
                                }
                            } else if let Some(_topping) = table.topping() {
                                // TOPPING TABLE
                                info!("TRYING TO INSERT TOPPING!");
                            } else if table.delivery() {
                                // DELIVERY TABLE
                                info!("TRYING TO DELIVER ORDER!");
                                if let Some(Either::Right(order)) = &player.inventory {
                                    info!("DELIVERED {:#?}!", order);
                                } else {
                                    info!("CANNOT DELIVER EMPTY OR SCOOP!");
                                }
                            } else {
                                // Empty Table, can place in/complete orders.
                            }
                        }
                    }
                    break;
                }
            }
        }
    }
}
