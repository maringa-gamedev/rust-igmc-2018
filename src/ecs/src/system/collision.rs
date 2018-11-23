use amethyst::{
    core::{timing::Time, transform::Transform},
    ecs::prelude::{Entities, Join, Read, ReadStorage, System, WriteStorage},
};
use crate::component::*;
use nk_data::*;
use either::*;
use nalgebra::{Isometry2, Vector2};
use ncollide2d::query::{self};

pub struct CollisionSystem;

impl<'s> System<'s> for CollisionSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Hitbox>,
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Velocity>,
        ReadStorage<'s, Solid>,
        ReadStorage<'s, Direction>,
        Read<'s, Time>,
    );

    fn run(
        &mut self,
        (entities, hitboxes, mut transforms, velocities, solids, directions, time): Self::SystemData,
    ) {
        for (e, e_hitbox, e_velocity, _) in (&*entities, &hitboxes, &velocities, !&solids).join() {
            for (o, o_hitbox, _) in (&*entities, &hitboxes, &solids).join() {
                if e != o {
                    let o_pos = {
                        let o_transform = transforms.get(o).unwrap();
                        Isometry2::new(
                            Vector2::new(o_transform.translation.x + o_hitbox.offset.x,
                                         o_transform.translation.y + o_hitbox.offset.y),
                            nalgebra::zero(),
                        )
                    };
                    let e_pos = {
                        let e_transform = transforms.get(e).unwrap();
                        Isometry2::new(
                            Vector2::new(e_transform.translation.x + e_hitbox.offset.x,
                                         e_transform.translation.y + e_hitbox.offset.y),
                            nalgebra::zero(),
                        )
                    };
                    if let Some(result) = match (&o_hitbox.shape, &e_hitbox.shape)  {
                            (Either::Left(o), Either::Left(e)) => query::contact(&o_pos, o, &e_pos, e, 1.0),
                            (Either::Left(o), Either::Right(e)) => query::contact(&o_pos, o, &e_pos, e, 1.0),
                            (Either::Right(o), Either::Left(e)) => query::contact(&o_pos, o, &e_pos, e, 1.0),
                            (Either::Right(o), Either::Right(e)) => query::contact(&o_pos, o, &e_pos, e, 1.0),
                        }
                     {
                        let depth = result.depth;
                        if depth > 0.0 {
                            let t = transforms.get_mut(e).unwrap();
                            let d = directions.get(e).unwrap();
                            let curr = d.current;
                            let prev = d.previous;
                            match (curr, prev) {
                                (Cardinal::East, _) => t.translation[0] -= depth,
                                (Cardinal::West, _) => t.translation[0] += depth,
                                (Cardinal::North, _) => t.translation[1] -= depth,
                                (Cardinal::South, _) => t.translation[1] += depth,

                                (Cardinal::NorthWest, _) => {
                                    t.translation[0] += depth;
                                    t.translation[1] -= depth;
                                }
                                (Cardinal::NorthEast, _) => {
                                    t.translation[0] -= depth;
                                    t.translation[1] -= depth;
                                }
                                (Cardinal::SouthWest, _) => {
                                    t.translation[0] += depth;
                                    t.translation[1] += depth;
                                }
                                (Cardinal::SouthEast, _) => {
                                    t.translation[0] -= depth;
                                    t.translation[1] += depth;
                                }

                                // TODO: Better correction when going diagonally.
                                
                                //(Cardinal::NorthWest, Some(Cardinal::West)) => {
                                //t.translation[0] += depth;
                                //t.translation[1] -= depth;
                                //}
                                //(Cardinal::NorthWest, Some(Cardinal::North)) => {
                                //t.translation[0] += depth
                                //}
                                //(Cardinal::NorthWest, _) => t.translation[0] += depth,

                                //(Cardinal::NorthEast, Some(Cardinal::East)) => {
                                //t.translation[0] += depth
                                //}
                                //(Cardinal::NorthEast, Some(Cardinal::North)) => {
                                //t.translation[0] += depth
                                //}
                                //(Cardinal::NorthEast, _) => t.translation[0] += depth,

                                //(Cardinal::SouthWest, Some(Cardinal::West)) => {
                                //t.translation[0] += depth
                                //}
                                //(Cardinal::SouthWest, Some(Cardinal::South)) => {
                                //t.translation[0] += depth
                                //}
                                //(Cardinal::SouthWest, _) => t.translation[0] += depth,

                                //(Cardinal::SouthEast, Some(Cardinal::East)) => {
                                //t.translation[0] += depth
                                //}
                                //(Cardinal::SouthEast, Some(Cardinal::South)) => {
                                //t.translation[0] += depth
                                //}
                                //(Cardinal::SouthEast, _) => t.translation[0] += depth,
                            }
                        }
                    }
                }
            }
        }
    }
}
