use amethyst::{
    core::{
        cgmath::*,
        timing::Time,
        transform::{Parent, Transform},
    },
    ecs::prelude::*,
    renderer::SpriteRender,
};
use crate::component::*;
use itertools::*;
use log::*;
use nk_data::*;

const W: usize = 10;
const H: usize = 11;

#[derive(Default)]
pub struct AutotileSystem {
    map: [[Option<Entity>; W]; H],
    turn: bool,
}

impl<'s> System<'s> for AutotileSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Parent>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Table>,
        WriteStorage<'s, SpriteRender>,
    );

    fn run(&mut self, (entities, parents, transforms, tables, mut sprites): Self::SystemData) {
        for j in 0..H {
            for i in 0..W {
                self.map[j][i] = None;
            }
        }

        let grouped = (&*entities, &transforms, &tables, &parents)
            .join()
            .group_by(|(_, _, _, p)| p.entity.id());

        for (_, g) in grouped.into_iter() {
            if self.turn {
                for (e, transform, table, _) in g {
                    if table.empty() {
                        let x = (transform.translation.x / 16.0) as usize;
                        let y = (transform.translation.y / 16.0) as usize;
                        self.map[y][x] = Some(e);
                    }
                }
            }
            self.turn = !self.turn;
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

        self.turn = !self.turn;
    }
}
