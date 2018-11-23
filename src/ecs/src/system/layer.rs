use amethyst::{
    core::transform::Transform,
    ecs::prelude::{Join, ReadStorage, System, WriteStorage},
};
use crate::component::*;

pub struct LayerSystem;

impl<'s> System<'s> for LayerSystem {
    type SystemData = (ReadStorage<'s, Layered>, WriteStorage<'s, Transform>);

    fn run(&mut self, (layereds, mut transforms): Self::SystemData) {
        for (o, _) in (&mut transforms, &layereds).join() {
            o.translation.z = -(o.translation.y - 16.0);
        }
    }
}
