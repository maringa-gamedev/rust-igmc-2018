use amethyst::{
    core::timing::Time,
    ecs::prelude::{Entities, Join, Read, System, WriteStorage},
};
use crate::component::*;
use either::*;

pub struct MeltSystem;

impl<'s> System<'s> for MeltSystem {
    type SystemData = (Entities<'s>, WriteStorage<'s, Player>, Read<'s, Time>);

    fn run(&mut self, (entities, mut players, time): Self::SystemData) {
        let ds = time.delta_seconds();
        for (_e, player) in (&*entities, &mut players).join() {
            if let Some(Either::Right(o)) = &mut player.inventory {
                o.update_delivery(ds);
                if o.has_melted() {
                    player.inventory = None;
                }
            }
        }
    }
}
