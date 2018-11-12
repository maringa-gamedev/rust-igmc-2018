use amethyst::{
    core::{timing::Time, transform::Transform},
    ecs::prelude::{Entities, Join, Read, ReadStorage, System, WriteStorage},
};
use crate::ecs::*;
use log::*;
use nalgebra::{Isometry2, Vector2};
use ncollide2d::query::{self, *};

pub struct MovementSystem;

const EPSILON: f32 = 0.33;

impl<'s> System<'s> for MovementSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Velocity>,
        WriteStorage<'s, Direction>,
        Read<'s, Time>,
    );

    fn run(
        &mut self,
        (entities, mut transforms, mut velocities, mut directions, time): Self::SystemData,
    ) {
        let ds = time.delta_seconds();
        for (e, transform, velocity) in (&*entities, &mut transforms, &mut velocities).join() {
            transform.translation.x += velocity.current.x * ds;
            transform.translation.y += velocity.current.y * ds;

            if let Some(direction) = directions.get_mut(e) {
                match velocity.current.x.abs() > 0.0 || velocity.current.y.abs() > 0.0 {
                    true => direction.current_anim = String::from("walk"),
                    false => direction.current_anim = String::from("idle"),
                }
            }

            if let Step::Absolute = velocity.step {
                velocity.current.x = 0.0;
                velocity.current.y = 0.0;
            }
        }
    }
}
