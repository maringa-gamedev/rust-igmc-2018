use amethyst::{
    core::timing::Time,
    ecs::prelude::{Read, System, Write},
};
use nk_data::*;

pub struct TimerSystem;

impl<'s> System<'s> for TimerSystem {
    type SystemData = (Write<'s, Match>, Read<'s, Time>);

    fn run(&mut self, (mut match_data, time): Self::SystemData) {
        match_data.timer -= time.delta_seconds();
    }
}
