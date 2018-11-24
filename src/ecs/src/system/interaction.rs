use amethyst::{
    assets::AssetStorage,
    audio::{output::Output, Source},
    core::{
        timing::Time,
        transform::{ParentHierarchy, Transform},
    },
    ecs::prelude::{Entities, Entity, Join, Read, ReadExpect, ReadStorage, System, WriteStorage},
    renderer::{Hidden, SpriteRender},
};
use crate::component::*;
use either::*;
use log::*;
use nk_data::*;
use shred_derive::*;
use std::ops::Deref;

pub struct InteractionSystem;

#[derive(SystemData)]
pub struct InteractionSystemData<'s> {
    parent_hierarchy: ReadExpect<'s, ParentHierarchy>,
    entities: Entities<'s>,
    players: WriteStorage<'s, Player>,
    inputs: WriteStorage<'s, Input>,
    transforms: WriteStorage<'s, Transform>,
    hitboxes: ReadStorage<'s, Hitbox>,
    directions: ReadStorage<'s, Direction>,
    interacts: WriteStorage<'s, Interact>,
    flavor_interactions: WriteStorage<'s, FlavorInteraction>,
    preparation_interactions: WriteStorage<'s, PreparationInteraction>,
    topping_interactions: WriteStorage<'s, ToppingInteraction>,
    tables: WriteStorage<'s, Table>,
    sprites: WriteStorage<'s, SpriteRender>,
    hiddens: WriteStorage<'s, Hidden>,
    start_pieces: WriteStorage<'s, StartBarPiece>,
    middle_pieces: WriteStorage<'s, MiddleBarPiece>,
    end_pieces: WriteStorage<'s, EndBarPiece>,
    backgrounds: WriteStorage<'s, BarBackground>,
    foregrounds: WriteStorage<'s, BarForeground>,
    hold_keys: WriteStorage<'s, HoldKey>,
    sequence_keys: WriteStorage<'s, SequenceKey>,
    alternative_keys: WriteStorage<'s, AlternativeKey>,
    anims: Read<'s, Animations>,
    storage: Read<'s, AssetStorage<Source>>,
    sounds: ReadExpect<'s, Sounds>,
    audio_output: Option<Read<'s, Output>>,
    time: Read<'s, Time>,
}

impl<'s> System<'s> for InteractionSystem {
    type SystemData = InteractionSystemData<'s>;

    fn run(
        &mut self,
        InteractionSystemData {
            parent_hierarchy,
            entities,
            mut players,
            mut inputs,
            mut transforms,
            hitboxes,
            directions,
            mut interacts,
            mut flavor_interactions,
            mut preparation_interactions,
            mut topping_interactions,
            mut tables,
            mut sprites,
            mut hiddens,
            mut start_pieces,
            mut middle_pieces,
            mut end_pieces,
            mut backgrounds,
            mut foregrounds,
            mut hold_keys,
            mut sequence_keys,
            mut alternative_keys,
            anims,
            storage,
            sounds,
            audio_output,
            time,
        }: Self::SystemData,
    ) {
        for (mut player, mut input) in (&mut players, &mut inputs).join() {
            if let Some(e) = player.interaction {
                if let Some(ref mut fi) = flavor_interactions.get_mut(e) {
                    // Player has interaction and it is flavor.
                    if match fi.key {
                        InteractionKey::North => input.wants_north,
                        InteractionKey::South => input.wants_south,
                        InteractionKey::West => input.wants_west,
                        InteractionKey::East => input.wants_east,
                    } {
                        fi.add_progress(time.delta_seconds());
                        if fi.is_complete() {
                            // Reset button inputs
                            input.wants_north = false;
                            input.wants_south = false;
                            input.wants_west = false;
                            input.wants_east = false;
                            // Insert flavor into player inventory
                            player.inventory = Some(Either::Left(fi.flavor.clone()));
                            // Delete interaction entity
                            player.interaction = None;
                            for child in parent_hierarchy.children(e) {
                                entities.delete(*child).unwrap();
                            }
                            entities.delete(e).unwrap();
                            // Play sound
                            play_pickup(
                                &*sounds,
                                &storage,
                                audio_output.as_ref().map(|o| o.deref()),
                            );
                            continue;
                        }
                    } else {
                        fi.remove_progress(time.delta_seconds());
                    }
                    for child in parent_hierarchy.children(e) {
                        if !self.update_child(
                            child,
                            &mut foregrounds,
                            &mut transforms,
                            &mut hiddens,
                            &mut start_pieces,
                            &mut middle_pieces,
                            &mut end_pieces,
                            fi.progress,
                        ) {};
                    }
                } else if let Some(ref mut pi) = preparation_interactions.get_mut(e) {
                    // Player has interaction and it is preparation.
                    let key = if input.wants_north {
                        Some(InteractionKey::North)
                    } else if input.wants_south {
                        Some(InteractionKey::South)
                    } else if input.wants_west {
                        Some(InteractionKey::West)
                    } else if input.wants_east {
                        Some(InteractionKey::East)
                    } else {
                        None
                    };
                    if let Some(key) = key {
                        input.wants_north = false;
                        input.wants_south = false;
                        input.wants_west = false;
                        input.wants_east = false;
                        pi.process_key(key);
                        if pi.is_complete() {
                            let mut order = tables.get_mut(pi.table).unwrap().extract_order();
                            order.completed = true;
                            player.inventory = Some(Either::Right(order));
                            player.interaction = None;
                            for child in parent_hierarchy.children(e) {
                                entities.delete(*child).unwrap();
                            }
                            entities.delete(e).unwrap();
                            continue;
                        }
                    }
                    for child in parent_hierarchy.children(e) {
                        if !self.update_child(
                            child,
                            &mut foregrounds,
                            &mut transforms,
                            &mut hiddens,
                            &mut start_pieces,
                            &mut middle_pieces,
                            &mut end_pieces,
                            pi.progress,
                        ) {
                            if let Some(SequenceKey(index)) = sequence_keys.get(*child) {
                                let anim = &anims.animations[&format!(
                                    "prompt_{}",
                                    pi.current_key(*index, &player.gamepad_style)
                                )];
                                let sprite = sprites.get_mut(*child).unwrap();
                                sprite.sprite_sheet = anim.obtain_handle();
                                sprite.sprite_number = anim.get_frame();
                            }
                        };
                    }
                } else if let Some(ref mut ti) = topping_interactions.get_mut(e) {
                    // Player has interaction and it is toppings.
                    ti.remove_progress(time.delta_seconds());
                    if let Some(key) = if input.wants_north {
                        Some(InteractionKey::North)
                    } else if input.wants_south {
                        Some(InteractionKey::South)
                    } else if input.wants_west {
                        Some(InteractionKey::West)
                    } else if input.wants_east {
                        Some(InteractionKey::East)
                    } else {
                        None
                    } {
                        input.wants_north = false;
                        input.wants_south = false;
                        input.wants_west = false;
                        input.wants_east = false;
                        ti.process_key(key);
                        if ti.is_complete() {
                            // This should never panic assuming the logic in the interact system is
                            // correct.
                            // There we should never allow a player with something order than a
                            // valid order (prepared, not already 'topped' and 3 or less
                            // ingredients) to enter this interaction (generate the entity).
                            player
                                .inventory
                                .as_mut()
                                .unwrap()
                                .as_mut()
                                .right()
                                .unwrap()
                                .insert_topping(ti.topping.clone());
                            player.interaction = None;
                            for child in parent_hierarchy.children(e) {
                                entities.delete(*child).unwrap();
                            }
                            entities.delete(e).unwrap();
                            continue;
                        }
                    }
                    for child in parent_hierarchy.children(e) {
                        if !self.update_child(
                            child,
                            &mut foregrounds,
                            &mut transforms,
                            &mut hiddens,
                            &mut start_pieces,
                            &mut middle_pieces,
                            &mut end_pieces,
                            ti.progress,
                        ) {}
                    }
                } else {
                    error!("SOMETHING IS WRONG, INTERACTION OF NO TYPE");
                }
            }
        }
    }
}

impl<'s> InteractionSystem {
    fn update_child(
        &self,
        child: &'s Entity,
        foregrounds: &mut WriteStorage<'s, BarForeground>,
        transforms: &mut WriteStorage<'s, Transform>,
        hiddens: &mut WriteStorage<'s, Hidden>,
        start_pieces: &mut WriteStorage<'s, StartBarPiece>,
        middle_pieces: &mut WriteStorage<'s, MiddleBarPiece>,
        end_pieces: &mut WriteStorage<'s, EndBarPiece>,
        progress: f32,
    ) -> bool {
        if let Some(_) = foregrounds.get(*child) {
            let transform = transforms.get_mut(*child).unwrap();
            let progress = (progress * 40.0).round() as usize;

            if let Some(_) = start_pieces.get(*child) {
                match progress {
                    0...3 => {
                        if let None = hiddens.get(*child) {
                            hiddens.insert(*child, Hidden).unwrap();
                        }
                    }
                    _ => {
                        if let Some(_) = hiddens.get(*child) {
                            hiddens.remove(*child);
                        }
                    }
                };
            } else if let Some(_) = middle_pieces.get(*child) {
                match progress {
                    4...40 => {
                        if let Some(_) = hiddens.get(*child) {
                            hiddens.remove(*child);
                        }
                        let progress = (progress - 4).min(32) as f32;
                        transform.translation.x = progress / 2.0 - 16.0;
                        transform.scale.x = progress / 8.0;
                    }
                    _ => {
                        if let None = hiddens.get(*child) {
                            hiddens.insert(*child, Hidden).unwrap();
                        }
                    }
                };
            } else if let Some(_) = end_pieces.get(*child) {
                match progress {
                    0...35 => {
                        if let None = hiddens.get(*child) {
                            hiddens.insert(*child, Hidden).unwrap();
                        }
                    }
                    _ => {
                        if let Some(_) = hiddens.get(*child) {
                            hiddens.remove(*child);
                        }
                    }
                };
            } else {
                return false;
            }
            true
        } else {
            false
        }
    }
}
