use amethyst::{core::cgmath::*, ecs::prelude::*};
use crate::{component::*, controller::*, frange::*};
use log::info;
use nk_data::*;
use std::{collections::HashMap, sync::*};

pub struct ControlSystem;

impl<'s> System<'s> for ControlSystem {
    type SystemData = (
        Entities<'s>,
        Read<'s, Arc<Mutex<HashMap<usize, Controller>>>>,
        ReadStorage<'s, Player>,
        WriteStorage<'s, Input>,
        WriteStorage<'s, Direction>,
        Write<'s, Animations>,
    );

    fn run(
        &mut self,
        (entities, controllers, players, mut inputs, mut directions, mut animations): Self::SystemData,
    ) {
        for (e, player, input) in (&*entities, &players, &mut inputs).join() {
            let mut controllers = controllers.lock().unwrap();
            let entry = controllers
                .entry(player.gamepad_index)
                .or_insert(Controller::new(player.gamepad_index));

            info!(
                "<{}> Movement Direction: {:?}",
                player.gamepad_index, entry.left_axis
            );
            info!(
                "<{}> Aim Direction: {:?}",
                player.gamepad_index, entry.right_axis
            );
            info!("<{}> Actions: {:?}", player.gamepad_index, entry.actions);
            info!(
                "<{}> Player Inventory: {:?}",
                player.gamepad_index, player.inventory
            );

            if entry.left_axis.relative_ne(
                &Vector3::new(0.0, 0.0, 0.0),
                Vector3::<f32>::default_epsilon(),
                Vector3::<f32>::default_max_relative(),
            ) {
                let vec = Vector2::new(entry.left_axis.x, entry.left_axis.y).normalize();
                let angle: Deg<f32> = Angle::atan2(vec.y, vec.x);

                info!("Angle: {}", angle.0);
                let new_angle = match angle.0 {
                    x if x.in_range(157.5, 181.0) => Cardinal::West,
                    x if x.in_range(112.5, 157.5) => Cardinal::NorthWest,
                    x if x.in_range(67.5, 112.5) => Cardinal::North,
                    x if x.in_range(22.5, 67.5) => Cardinal::NorthEast,
                    x if x.in_range(-22.5, 22.5) => Cardinal::East,
                    x if x.in_range(-67.5, -22.5) => Cardinal::SouthEast,
                    x if x.in_range(-112.5, -67.5) => Cardinal::South,
                    x if x.in_range(-157.5, -112.5) => Cardinal::SouthWest,
                    x if x.in_range(-181.0, -157.5) => Cardinal::West,

                    // Atan2 should never output anything above 180deg.
                    x => panic!("Invalid direction, angle {}!", x),
                };
                if Some(new_angle) != input.wants_to_move {
                    input.last_moved_direction = input.wants_to_move;

                    //for (_, anim) in &mut animations.animations {
                    //anim.reset();
                    //}
                }
                input.wants_to_move = Some(new_angle);

                if let Some(dir) = directions.get_mut(e) {
                    dir.previous = Some(dir.current);
                    dir.current = new_angle;
                }
            } else {
                input.wants_to_move = None;
            }

            input.wants_to_interact = entry.actions[0];
            //input.wants_to_shoot = entry.triggers[1];
            //input.wants_to_shield = entry.shoulders[0] || entry.shoulders[1];
        }
    }
}
