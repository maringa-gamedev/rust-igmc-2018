use amethyst::{
    core::{timing::Time, transform::Transform},
    ecs::prelude::{Join, Read, System, WriteStorage},
};
use crate::component::*;

pub struct BackgroundAnimationSystem;

const INTERVAL: f32 = 0.015625;
const STEP: f32 = 0.015625;

impl<'s> System<'s> for BackgroundAnimationSystem {
    type SystemData = (
        WriteStorage<'s, Background>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut backgrounds, mut transforms, time): Self::SystemData) {
        for (b, t) in (&mut backgrounds, &mut transforms).join() {
            b.timer += time.delta_seconds();
            while b.timer >= INTERVAL {
                t.translation.x = if t.translation.x + STEP > b.to.x {
                    b.from.x
                } else {
                    t.translation.x + STEP
                };
                t.translation.y = if t.translation.y + STEP > b.to.y {
                    b.from.y
                } else {
                    t.translation.y + STEP
                };
                b.timer -= INTERVAL;
            }
        }
    }
}
