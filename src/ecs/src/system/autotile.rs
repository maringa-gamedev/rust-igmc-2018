use amethyst::{
    core::{cgmath::*, timing::Time, transform::Transform},
    ecs::prelude::*,
    renderer::SpriteRender,
};
use crate::component::*;
use log::*;
use nk_data::*;

const W: usize = 10;
const H: usize = 11;

#[derive(Default)]
pub struct AutotileSystem {
    map: [[Option<Entity>; W]; H],
}

impl<'s> System<'s> for AutotileSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Table>,
        WriteStorage<'s, SpriteRender>,
    );

    fn run(&mut self, (entities, transforms, tables, mut sprites): Self::SystemData) {
        for j in 0..H {
            for i in 0..W {
                self.map[j][i] = None;
            }
        }

        for (e, transform, table) in (&*entities, &transforms, &tables).join() {
            if table.empty() {
                let x = ((transform.translation.x - MAP_OFFSET_X) / 16.0) as usize;
                let y = ((transform.translation.y - MAP_OFFSET_Y) / 16.0) as usize;
                self.map[y][x] = Some(e);
            }
        }

        for j in 0..H {
            for i in 0..W {
                if let Some(e) = self.map[j][i] {
                    let mut sum = 0;
                    if let Some(r) = self.map.get(j + 1) {
                        if let Some(Some(_)) = r.get(i) {
                            sum += 1;
                        }
                    }
                    if j > 0 {
                        if let Some(r) = self.map.get(j - 1) {
                            if let Some(Some(_)) = r.get(i) {
                                sum += 8;
                            }
                        }
                    }
                    if let Some(r) = self.map.get(j) {
                        if let Some(Some(_)) = r.get(i + 1) {
                            sum += 4;
                        }
                        if i > 0 {
                            if let Some(Some(_)) = r.get(i - 1) {
                                sum += 2;
                            }
                        }
                    }
                    sprites.get_mut(e).unwrap().sprite_number = sum;
                }
            }
        }
    }
}
