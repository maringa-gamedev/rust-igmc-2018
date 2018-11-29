use amethyst::{
    core::{
        timing::Time,
        transform::{Parent, Transform},
    },
    ecs::prelude::{Entities, Join, Read, ReadStorage, System, WriteStorage},
    renderer::{Hidden, SpriteRender},
};
use crate::component::*;
use either::*;
use log::*;
use nalgebra::{Isometry2, Vector2};
use ncollide2d::query::{self, *};
use nk_data::*;

pub struct InventoryRenderSystem;

impl<'s> System<'s> for InventoryRenderSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Parent>,
        ReadStorage<'s, Player>,
        ReadStorage<'s, InventoryItem>,
        WriteStorage<'s, SpriteRender>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Hidden>,
        Read<'s, Animations>,
        Read<'s, Definitions>,
    );

    fn run(
        &mut self,
        (
            entities,
            parents,
            players,
            inventory_items,
            mut sprites,
            mut transforms,
            mut hiddens,
            anims,
            defs,
        ): Self::SystemData,
    ) {
        let anims = &anims.animations;
        for (e, transform, parent, sprite, inv) in (
            &*entities,
            &mut transforms,
            &parents,
            &mut sprites,
            &inventory_items,
        )
            .join()
        {
            if let Some(item_parent) = parents.get(parent.entity) {
                if let Some(player) = players.get(item_parent.entity) {
                    if let Some(_) = &player.interaction {
                        if let None = hiddens.get(e) {
                            hiddens.insert(e, Hidden).unwrap();
                        }
                    } else {
                        match &player.inventory {
                            Some(Either::Left(f)) => match inv.0 {
                                0 => {
                                    if let Some(_) = hiddens.get(e) {
                                        hiddens.remove(e);
                                    }
                                    let key = &defs
                                        .flavors()
                                        .find(|a| a.index == *f)
                                        .expect(&format!(
                                            "Flavor Definition must include index <{:?}>",
                                            f
                                        ))
                                        .key;
                                    let anim = &anims
                                        .get(&format!("{}_scoop", &key))
                                        .expect(&format!("{}_scoop not found", &key));

                                    sprite.sprite_sheet = anim.obtain_handle();
                                    sprite.sprite_number = anim.get_frame();
                                    transform.translation.x = 0.0;
                                    transform.translation.y = 0.0;
                                    transform.translation.y = 1.0;
                                }
                                _ => {
                                    if let None = hiddens.get(e) {
                                        hiddens.insert(e, Hidden).unwrap();
                                    }
                                }
                            },
                            Some(Either::Right(o)) => {
                                let (handle, frame) = if inv.0 == 6 {
                                    let progress = o.melt_percent();
                                    let a = &anims.get("color_progress").unwrap();
                                    (a.obtain_handle(), a.get_frame_at(progress, false))
                                } else if inv.0 == 5 {
                                    let (first, second) = o.get_topping_key(&defs);
                                    if let Some(a) = anims.get(&first) {
                                        (a.obtain_handle(), a.get_frame())
                                    } else {
                                        let a = &anims.get(&second).unwrap();
                                        (a.obtain_handle(), a.get_frame())
                                    }
                                } else {
                                    let key = &match inv.0 {
                                        0 => o.get_preparation_key(&defs),
                                        1 => o.get_flavor_a_key(&defs),
                                        2 => o.get_flavor_b_key(&defs),
                                        3 => o.get_flavor_c_key(&defs),
                                        4 => o.get_flavor_d_key(&defs),
                                        _ => std::unreachable!(),
                                    };
                                    let a = &anims.get(key).unwrap();
                                    (a.obtain_handle(), a.get_frame())
                                };
                                sprite.sprite_sheet = handle;
                                sprite.sprite_number = frame;

                                let flavor_count = o.flavor_count();
                                let preparation_def = defs
                                    .preparations()
                                    .find(|x| x.index == o.preparation)
                                    .unwrap();

                                let (x, y) = match inv.0 {
                                    0 => (0.0, 0.0),
                                    1 => match flavor_count {
                                        1 => preparation_def.offsets.one[0],
                                        2 => preparation_def.offsets.two[0],
                                        3 => preparation_def.offsets.three[0],
                                        4 => preparation_def.offsets.four[0],
                                        _ => std::unreachable!(),
                                    },
                                    2 => match flavor_count {
                                        1 => (0.0, 0.0),
                                        2 => preparation_def.offsets.two[1],
                                        3 => preparation_def.offsets.three[1],
                                        4 => preparation_def.offsets.four[1],
                                        _ => std::unreachable!(),
                                    },
                                    3 => match flavor_count {
                                        1 => (0.0, 0.0),
                                        2 => (0.0, 0.0),
                                        3 => preparation_def.offsets.three[2],
                                        4 => preparation_def.offsets.four[2],
                                        _ => std::unreachable!(),
                                    },
                                    4 => match flavor_count {
                                        1 => (0.0, 0.0),
                                        2 => (0.0, 0.0),
                                        3 => (0.0, 0.0),
                                        4 => preparation_def.offsets.four[3],
                                        _ => std::unreachable!(),
                                    },
                                    5 => (0.0, 0.0),
                                    6 => (0.0, 12.0),
                                    _ => std::unreachable!(),
                                };

                                transform.translation.x = x;
                                transform.translation.y = y;

                                if let Some(_) = hiddens.get(e) {
                                    hiddens.remove(e);
                                }
                            }
                            _ => {
                                if let None = hiddens.get(e) {
                                    hiddens.insert(e, Hidden).unwrap();
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
