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
        ReadStorage<'s, Hitbox>,
        ReadStorage<'s, Solid>,
        WriteStorage<'s, Velocity>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, PixelPerfect>,
        Read<'s, Time>,
    );

    fn run(
        &mut self,
        (entities, hitboxes, solids, mut velocities, mut transforms, mut pixels, time): Self::SystemData,
    ) {
        let ds = time.delta_seconds();
        for (e, hitbox, velocity) in (&*entities, &hitboxes, &mut velocities).join() {
            let t = transforms.get_mut(e).unwrap();
            t.translation[0] += velocity.current[0] * ds;
            t.translation[1] += velocity.current[1] * ds;

            if let Step::Absolute = velocity.step {
                velocity.current[0] = 0.0;
                velocity.current[1] = 0.0;
            }
            /*
             *            let next_pos = {
             *                let p = pixels.get(e).unwrap();
             *                Isometry2::new(
             *                    Vector2::new(
             *                        (p.position[0] + velocity.current[0] * ds).round(),
             *                        (p.position[1] + velocity.current[1] * ds).round(),
             *                        //(p.position[0] + velocity.current[0] * ds).round() - hitbox.shape.w,
             *                        //(p.position[1] + velocity.current[1] * ds).round() - hitbox.shape.height / 2.0,
             *                    ),
             *                    nalgebra::zero(),
             *                )
             *            };
             *
             *            let my_shape = &hitbox.shape;
             *            let mut can_move = true;
             *
             *            for (s, hitbox, _) in (&*entities, &hitboxes, &solids).join() {
             *                let solid_pos = {
             *                    let t = transforms.get(s).unwrap();
             *                    Isometry2::new(
             *                        Vector2::new(t.translation.x, t.translation.y),
             *                        nalgebra::zero(),
             *                    )
             *                };
             *                let solid_shape = &hitbox.shape;
             *
             *                if let Proximity::Intersecting =
             *                    query::proximity(&next_pos, my_shape, &solid_pos, solid_shape, 1.0)
             *                {
             *                    info!("CANNOT MOVE HERE");
             *                    can_move = false;
             *                }
             *            }
             *
             *            if can_move {
             *                let p = pixels.get_mut(e).unwrap();
             *                let t = transforms.get_mut(e).unwrap();
             *
             *                p.position[0] += velocity.current[0] * ds;
             *                p.position[1] += velocity.current[1] * ds;
             *
             *                //t.translation[0] = p.position[0] - (p.position[0] % -0.5);
             *                //t.translation[1] = p.position[1] - (p.position[1] % -0.5);
             *
             *                t.translation[0] = p.position[0].round();
             *                t.translation[1] = p.position[1].round();
             *            }
             *
             *            if let Step::Absolute = velocity.step {
             *                velocity.current[0] = 0.0;
             *                velocity.current[1] = 0.0;
             *            }
             */
        }
    }
}
