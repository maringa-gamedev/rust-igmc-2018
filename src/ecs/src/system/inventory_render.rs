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
        WriteStorage<'s, Hidden>,
        Read<'s, Animations>,
        Read<'s, Definitions>,
    );

    fn run(
        &mut self,
        (entities, parents, players, inventory_items, mut sprites, mut hiddens, anims, defs): Self::SystemData,
    ) {
        let anims = &anims.animations;
        for (e, parent, sprite, _) in (&*entities, &parents, &mut sprites, &inventory_items).join()
        {
            if let Some(player) = players.get(parent.entity) {
                match &player.inventory {
                    Some(Either::Left(f)) => {
                        //info!("Carrying scoop!");
                        if let Some(_) = hiddens.get(e) {
                            hiddens.remove(e);
                        }
                        let key = &defs
                            .flavors()
                            .find(|a| a.index == *f)
                            .expect(&format!("Flavor Definition must include index <{:?}>", f))
                            .key;
                        sprite.sprite_number = anims
                            .get(&format!("scoop_{}", &key))
                            .expect(&format!("scoop_{} not found", &key))
                            .get_frame();
                    }
                    Some(Either::Right(o)) => {
                        //info!("Carrying order!");
                        if let Some(_) = hiddens.get(e) {
                            hiddens.remove(e);
                        }
                        let key = o.get_key();
                        sprite.sprite_number = anims
                            .get(&format!("order_{}", &key))
                            .expect(&format!("order_{} not found", &key))
                            .get_frame();
                    }
                    _ => {
                        //info!("Carrying nothing!");
                        if let None = hiddens.get(e) {
                            hiddens.insert(e, Hidden).unwrap();
                        }
                    }
                }
            }
        }
    }
}
