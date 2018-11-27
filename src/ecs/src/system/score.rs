use amethyst::{
    core::transform::{Parent, Transform},
    ecs::prelude::{Join, Read, ReadStorage, System, WriteStorage},
    renderer::SpriteRender,
};
use crate::component::*;
use log::*;
use nk_data::*;

pub struct ScoreSystem;

impl<'s> System<'s> for ScoreSystem {
    type SystemData = (
        ReadStorage<'s, ScoreDigit>,
        ReadStorage<'s, TimerDigit>,
        WriteStorage<'s, SpriteRender>,
        Read<'s, Match>,
    );

    fn run(&mut self, (scores, timers, mut sprites, match_data): Self::SystemData) {
        if match_data.teams.len() > 0 {
            let (minute, second, score_left, score_right) = {
                let minute = match_data.timer.ceil() as usize / 60;
                let second = match_data.timer.ceil() as usize % 60;
                let score_left = match_data.teams[0].score as usize;
                let score_right = match_data.teams[1].score as usize;
                (minute, second, score_left, score_right)
            };

            for (score, sprite) in (&scores, &mut sprites).join() {
                let to_mod = 10usize.pow(8u32 - score.0 as u32);
                let to_div = 10usize.pow(7u32 - score.0 as u32);
                if score.1 {
                    sprite.sprite_number = (score_right % to_mod) / to_div;
                } else {
                    sprite.sprite_number = (score_left % to_mod) / to_div;
                }
            }

            for (timer, sprite) in (&timers, &mut sprites).join() {
                sprite.sprite_number = match timer.0 {
                    0 => minute / 10,
                    1 => minute % 10,
                    2 => second / 10,
                    3 => second % 10,
                    4 => 10,
                    _ => panic!("IMPOSSIBURU! TIMER HAS ONLY 5 CHARS"),
                };
            }
        }
    }
}
