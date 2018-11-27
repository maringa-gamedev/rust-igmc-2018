use amethyst::{
    core::transform::Parent,
    ecs::prelude::{Join, Read, ReadStorage, System, WriteStorage},
    renderer::SpriteRender,
};
use crate::component::*;
use log::*;
use nk_data::*;

pub struct OrdersSystem;

impl<'s> System<'s> for OrdersSystem {
    type SystemData = (
        ReadStorage<'s, Parent>,
        ReadStorage<'s, OrderSlot>,
        WriteStorage<'s, OrderIngredient>,
        WriteStorage<'s, SpriteRender>,
        Read<'s, Match>,
        Read<'s, Animations>,
        Read<'s, Definitions>,
    );

    fn run(
        &mut self,
        (parents, slots, mut ingredients, mut sprites, match_data, anims, defs): Self::SystemData,
    ) {
        if match_data.teams.len() > 0 {
            for (ingredient, sprite, parent) in (&mut ingredients, &mut sprites, &parents).join() {
                let my_slot = slots.get(parent.entity).unwrap();
                let OrderSlot(team_index, order_index) = my_slot;
                let my_team = &match_data.teams[*team_index];
                if let Some(my_order) = my_team.orders.get(*order_index) {
                    let key = format!(
                        "{}_item",
                        match my_order.possibility {
                            OrderPossibility::OneFlavorNoTopping(ref f1) => {
                                if ingredient.0 == 0 {
                                    defs.flavors().find(|f| f.index == *f1).unwrap().key.clone()
                                } else {
                                    String::from("empty")
                                }
                            }
                            OrderPossibility::TwoFlavorsNoTopping(ref f1, ref f2) => {
                                if ingredient.0 == 0 {
                                    defs.flavors().find(|f| f.index == *f1).unwrap().key.clone()
                                } else if ingredient.0 == 1 {
                                    defs.flavors().find(|f| f.index == *f2).unwrap().key.clone()
                                } else {
                                    String::from("empty")
                                }
                            }
                            OrderPossibility::ThreeFlavorsNoTopping(ref f1, ref f2, ref f3) => {
                                if ingredient.0 == 0 {
                                    defs.flavors().find(|f| f.index == *f1).unwrap().key.clone()
                                } else if ingredient.0 == 1 {
                                    defs.flavors().find(|f| f.index == *f2).unwrap().key.clone()
                                } else if ingredient.0 == 2 {
                                    defs.flavors().find(|f| f.index == *f3).unwrap().key.clone()
                                } else {
                                    String::from("empty")
                                }
                            }
                            OrderPossibility::FourFlavorsNoTopping(
                                ref f1,
                                ref f2,
                                ref f3,
                                ref f4,
                            ) => {
                                if ingredient.0 == 0 {
                                    defs.flavors().find(|f| f.index == *f1).unwrap().key.clone()
                                } else if ingredient.0 == 1 {
                                    defs.flavors().find(|f| f.index == *f2).unwrap().key.clone()
                                } else if ingredient.0 == 2 {
                                    defs.flavors().find(|f| f.index == *f3).unwrap().key.clone()
                                } else if ingredient.0 == 3 {
                                    defs.flavors().find(|f| f.index == *f4).unwrap().key.clone()
                                } else {
                                    String::from("empty")
                                }
                            }
                            OrderPossibility::OneFlavorWithTopping(ref f1, ref t1) => {
                                if ingredient.0 == 0 {
                                    defs.flavors().find(|f| f.index == *f1).unwrap().key.clone()
                                } else if ingredient.0 == 1 {
                                    defs.toppings()
                                        .find(|t| t.index == *t1)
                                        .unwrap()
                                        .key
                                        .clone()
                                } else {
                                    String::from("empty")
                                }
                            }
                            OrderPossibility::TwoFlavorsWithTopping(ref f1, ref f2, ref t1) => {
                                if ingredient.0 == 0 {
                                    defs.flavors().find(|f| f.index == *f1).unwrap().key.clone()
                                } else if ingredient.0 == 1 {
                                    defs.flavors().find(|f| f.index == *f2).unwrap().key.clone()
                                } else if ingredient.0 == 2 {
                                    defs.toppings()
                                        .find(|t| t.index == *t1)
                                        .unwrap()
                                        .key
                                        .clone()
                                } else {
                                    String::from("empty")
                                }
                            }
                            OrderPossibility::ThreeFlavorsWithTopping(
                                ref f1,
                                ref f2,
                                ref f3,
                                ref t1,
                            ) => {
                                if ingredient.0 == 0 {
                                    defs.flavors().find(|f| f.index == *f1).unwrap().key.clone()
                                } else if ingredient.0 == 1 {
                                    defs.flavors().find(|f| f.index == *f2).unwrap().key.clone()
                                } else if ingredient.0 == 2 {
                                    defs.flavors().find(|f| f.index == *f3).unwrap().key.clone()
                                } else if ingredient.0 == 3 {
                                    defs.toppings()
                                        .find(|t| t.index == *t1)
                                        .unwrap()
                                        .key
                                        .clone()
                                } else {
                                    String::from("empty")
                                }
                            }
                        }
                    );
                    //info!("try item key = {}", key);
                    let anim = &anims.animations[&key];
                    sprite.sprite_sheet = anim.obtain_handle();
                    sprite.sprite_number = anim.get_frame();
                } else {
                    let anim = &anims.animations["empty_item"];
                    sprite.sprite_sheet = anim.obtain_handle();
                    sprite.sprite_number = anim.get_frame();
                }
            }
        }
    }
}
