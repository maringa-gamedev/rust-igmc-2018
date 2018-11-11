use amethyst::{
    core::{timing::Time, transform::Transform},
    ecs::prelude::{Entities, Join, Read, ReadStorage, System, WriteStorage},
};
use crate::{constants::*, ecs::*};
use itertools::Itertools;
use log::*;
use nalgebra::{Isometry2, Vector2};
use ncollide2d::query::{self, *};

pub struct LayerSystem;

impl<'s> System<'s> for LayerSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Layered>,
        WriteStorage<'s, Player>,
        WriteStorage<'s, Transform>,
    );

    fn run(&mut self, (entities, layereds, mut players, mut transforms): Self::SystemData) {
        for (o, _) in (&mut transforms, &layereds).join() {
            o.translation.z = -o.translation.y;
        }
        /*
         *let mut i = 0;
         *for (p, player) in (&mut transforms, &mut players)
         *    .join()
         *    .sorted_by(|(at, _), (bt, _)| at.translation.y.partial_cmp(&bt.translation.y).unwrap())
         *{
         *    p.translation.z = -1.0 * i as f32;
         *    player.layer = -1.0 * i as f32;
         *    i += 1;
         *}
         *for (o, _, _) in (&mut transforms, &layereds, !&players).join()
         *    .sorted_by(|(at, _, _), (bt, _, _)| at.translation.y.partial_cmp(&bt.translation.y).unwrap())
         *{
         *    o.translation.z = LAYER_MAP_GROUND;
         *}
         *for (o, _, _) in (&*entities, &layereds, !&players).join() {
         *    let o_y = {
         *        transforms.get(o).unwrap().translation.y;
         *    };
         *    for (p, player) in (&*entities, &players).join() {
         *        let p_y = {
         *            transforms.get(p).unwrap().translation.y;
         *        };
         *        let t = transforms.get_mut(o).unwrap();
         *        if o_y < p_y {
         *            t.translation.z = (player.layer + LAYER_PUT_IN_FRONT).max(t.translation.z);
         *        } else {
         *            t.translation.z = (player.layer + LAYER_PUT_BEHIND).min(t.translation.z);
         *        }
         *    }
         *}
         */
    }
}
