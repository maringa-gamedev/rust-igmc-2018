use amethyst::{
    core::{cgmath::*, timing::Time},
    ecs::prelude::*,
    renderer::SpriteRender,
};
use crate::component::*;
use log::*;
use nk_data::*;

pub struct AnimationSystem;

impl<'s> System<'s> for AnimationSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, AnimatedFloor>,
        ReadStorage<'s, AnimatedTable>,
        ReadStorage<'s, Interact>,
        ReadStorage<'s, Table>,
        WriteStorage<'s, Direction>,
        WriteStorage<'s, SpriteRender>,
        Read<'s, Time>,
        Write<'s, Animations>,
    );

    fn run(
        &mut self,
        (
            entities,
            floors,
            a_tables,
            interacts,
            tables,
            mut directions,
            mut sprites,
            time,
            mut animations,
        ): Self::SystemData,
    ) {
        let ds = time.delta_seconds();
        for (_, anim) in &mut animations.animations {
            anim.update_timer(ds);
        }
        let animations = &animations.animations;
        for (direction, sprite) in (&mut directions, &mut sprites).join() {
            sprite.sprite_number = match direction.current {
                Cardinal::North => {
                    animations[&format!("char_north_{}", &direction.current_anim)].get_frame()
                }
                Cardinal::NorthWest => {
                    animations[&format!("char_north_{}", &direction.current_anim)].get_frame()
                }
                Cardinal::NorthEast => {
                    animations[&format!("char_north_{}", &direction.current_anim)].get_frame()
                }

                Cardinal::South => {
                    animations[&format!("char_south_{}", &direction.current_anim)].get_frame()
                }
                Cardinal::SouthWest => {
                    animations[&format!("char_south_{}", &direction.current_anim)].get_frame()
                }
                Cardinal::SouthEast => {
                    animations[&format!("char_south_{}", &direction.current_anim)].get_frame()
                }

                Cardinal::West => {
                    animations[&format!("char_west_{}", &direction.current_anim)].get_frame()
                }
                Cardinal::East => {
                    animations[&format!("char_east_{}", &direction.current_anim)].get_frame()
                }
            };
        }

        for (floor, sprite) in (&floors, &mut sprites).join() {
            sprite.sprite_number = animations[&format!("{}_top", floor.0)].get_frame();
        }

        for (e, interact, a_table) in (&*entities, &interacts, &a_tables).join() {
            if let Some(_) = interact.highlighted_by {
                let s = sprites.get_mut(e).unwrap();
                s.sprite_number = animations[&format!("{}_side_open", a_table.0)].get_frame();

                let s = sprites.get_mut(interact.top).unwrap();
                s.sprite_number = animations[&format!("{}_top_open", a_table.0)].get_frame();
            } else {
                let s = sprites.get_mut(e).unwrap();
                s.sprite_number = animations[&format!("{}_side_close", a_table.0)].get_frame();

                let s = sprites.get_mut(interact.top).unwrap();
                s.sprite_number = animations[&format!("{}_top_close", a_table.0)].get_frame();
            }
        }
    }
}
