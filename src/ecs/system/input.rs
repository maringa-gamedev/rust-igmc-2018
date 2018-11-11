use amethyst::ecs::prelude::*;
use crate::ecs::*;

pub struct InputSystem;

impl<'s> System<'s> for InputSystem {
    type SystemData = (ReadStorage<'s, Input>, WriteStorage<'s, Velocity>);

    fn run(&mut self, (inputs, mut velocities): Self::SystemData) {
        for (input, velocity) in (&inputs, &mut velocities).join() {
            if let Some(dir) = input.wants_to_move {
                velocity.current[0] = velocity.velocity[0] * dir.get_x();
                velocity.current[1] = velocity.velocity[1] * dir.get_y();
            }
        }
    }
}
