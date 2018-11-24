use amethyst::{
    assets::AssetStorage,
    audio::{output::Output, Source},
    core::transform::{GlobalTransform, Parent, Transform},
    ecs::prelude::{Entities, Entity, Join, Read, ReadExpect, ReadStorage, System, WriteStorage},
    renderer::{Hidden, SpriteRender, Transparent},
};
use crate::component::*;
use either::*;
use itertools::Itertools;
use log::*;
use nalgebra::{Isometry2, Vector2};
use ncollide2d::{
    query::{self, *},
    shape::*,
};
use nk_data::*;
use shred_derive::*;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

pub struct InteractSystem;

type Palette = HashMap<String, image::Rgba<u8>>;
type ArcMutPalette = Arc<Mutex<Palette>>;

#[derive(SystemData)]
pub struct InteractSystemData<'s> {
    entities: Entities<'s>,
    players: WriteStorage<'s, Player>,
    inputs: WriteStorage<'s, Input>,
    transforms: WriteStorage<'s, Transform>,
    global_transforms: WriteStorage<'s, GlobalTransform>,
    hitboxes: ReadStorage<'s, Hitbox>,
    directions: ReadStorage<'s, Direction>,
    interacts: WriteStorage<'s, Interact>,
    tables: WriteStorage<'s, Table>,
    sprites: WriteStorage<'s, SpriteRender>,
    flavor_interactions: WriteStorage<'s, FlavorInteraction>,
    preparation_interactions: WriteStorage<'s, PreparationInteraction>,
    topping_interactions: WriteStorage<'s, ToppingInteraction>,
    parents: WriteStorage<'s, Parent>,
    hiddens: WriteStorage<'s, Hidden>,
    transparents: WriteStorage<'s, Transparent>,
    start_pieces: WriteStorage<'s, StartBarPiece>,
    middle_pieces: WriteStorage<'s, MiddleBarPiece>,
    end_pieces: WriteStorage<'s, EndBarPiece>,
    backgrounds: WriteStorage<'s, BarBackground>,
    foregrounds: WriteStorage<'s, BarForeground>,
    hold_keys: WriteStorage<'s, HoldKey>,
    sequence_keys: WriteStorage<'s, SequenceKey>,
    alternative_keys: WriteStorage<'s, AlternativeKey>,
    anims: Read<'s, Animations>,
    palette: Read<'s, ArcMutPalette>,
    storage: Read<'s, AssetStorage<Source>>,
    sounds: ReadExpect<'s, Sounds>,
    audio_output: Option<Read<'s, Output>>,
}

impl<'s> System<'s> for InteractSystem {
    type SystemData = InteractSystemData<'s>;

    fn run(
        &mut self,
        InteractSystemData {
            entities,
            mut players,
            mut inputs,
            mut transforms,
            mut global_transforms,
            hitboxes,
            directions,
            mut interacts,
            mut tables,
            mut sprites,
            mut flavor_interactions,
            mut preparation_interactions,
            mut topping_interactions,
            mut parents,
            mut hiddens,
            mut transparents,
            mut start_pieces,
            mut middle_pieces,
            mut end_pieces,
            mut backgrounds,
            mut foregrounds,
            mut hold_keys,
            mut sequence_keys,
            mut alternative_keys,
            anims,
            palette,
            storage,
            sounds,
            audio_output,
        }: Self::SystemData,
    ) {
        // Reset interaction highlight
        for interact in (&mut interacts).join() {
            interact.highlighted_by = None;
            //let top = sprites.get_mut(interact.top).unwrap();
            //top.sprite_number = interact.original;
        }

        for (player_entity, mut player, mut input, hitbox, direction) in (
            &*entities,
            &mut players,
            &mut inputs,
            &hitboxes,
            &directions,
        )
            .join()
        {
            if let Some(_) = player.interaction {
                info!("PLAYER IS ALREADY INTERACTING!");
            } else {
                let check_pos = {
                    let transform = transforms.get(player_entity).unwrap();
                    Isometry2::new(
                        Vector2::new(
                            transform.translation.x + direction.current.make_interaction_offset_x(),
                            transform.translation.y + direction.current.make_interaction_offset_y(),
                        ),
                        nalgebra::zero(),
                    )
                };
                let check_shape = match &hitbox.shape {
                    Either::Left(cuboid) => Cuboid::new(Vector2::new(
                        cuboid.half_extents().x * 0.9,
                        cuboid.half_extents().y * 0.9,
                    )),
                    Either::Right(ball) => {
                        Cuboid::new(Vector2::new(ball.radius() * 0.9, ball.radius() * 0.9))
                    }
                };
                let dir = direction.current;

                for (transform, hitbox, interact) in (&transforms, &hitboxes, &mut interacts)
                    .join()
                    .sorted_by(|(at, _, _), (bt, _, _)| {
                        // TODO: Could be improved by also looking at previous direction to determine
                        // interaction.
                        match dir {
                            Cardinal::North => bt.translation.y.partial_cmp(&at.translation.y),
                            Cardinal::NorthWest => bt.translation.y.partial_cmp(&at.translation.y),
                            Cardinal::NorthEast => bt.translation.y.partial_cmp(&at.translation.y),

                            Cardinal::South => at.translation.y.partial_cmp(&bt.translation.y),
                            Cardinal::SouthWest => at.translation.y.partial_cmp(&bt.translation.y),
                            Cardinal::SouthEast => at.translation.y.partial_cmp(&bt.translation.y),

                            Cardinal::West => at.translation.x.partial_cmp(&bt.translation.x),
                            Cardinal::East => bt.translation.x.partial_cmp(&at.translation.x),
                        }
                        .unwrap()
                    }) {
                    // If not already in use by someone else.
                    if let None = interact.highlighted_by {
                        // Make positions for comparison
                        let interact_pos = Isometry2::new(
                            Vector2::new(
                                transform.translation.x + hitbox.offset.x,
                                transform.translation.y + hitbox.offset.y,
                            ),
                            nalgebra::zero(),
                        );
                        let interact_shape = match &hitbox.shape {
                            Either::Left(cuboid) => cuboid,
                            Either::Right(_) => {
                                panic!("DO NOT USE BALLS FOR INTERACTION COLLISION")
                            }
                        };
                        //let top = sprites.get_mut(interact.top).unwrap();

                        if let Proximity::Intersecting = query::proximity(
                            &check_pos,
                            &check_shape,
                            &interact_pos,
                            interact_shape,
                            1.0,
                        ) {
                            interact.highlighted_by = Some(player.gamepad_index);
                            //top.sprite_number = interact.highlight;
                            info!("CAN INTERACT!");
                            if input.wants_to_interact {
                                if let Some(table) = tables.get_mut(interact.top) {
                                    if let Some(flavor) = table.flavor() {
                                        // FLAVOR TABLE
                                        info!("TRYING TO SCOOP FLAVOR!");
                                        if let Some(_item) = &player.inventory {
                                            info!("PLAYER CANNOT SCOOP AS INVENTORY IS FULL!");
                                        } else {
                                            let interaction_entity = self
                                                .create_flavor_interaction(
                                                    &entities,
                                                    &mut flavor_interactions,
                                                    &mut parents,
                                                    &mut hiddens,
                                                    &mut transparents,
                                                    &mut transforms,
                                                    &mut global_transforms,
                                                    &mut sprites,
                                                    &mut start_pieces,
                                                    &mut middle_pieces,
                                                    &mut end_pieces,
                                                    &mut backgrounds,
                                                    &mut foregrounds,
                                                    &mut hold_keys,
                                                    &anims,
                                                    flavor,
                                                    player_entity,
                                                    player.palette_key.clone(),
                                                    &player.gamepad_style,
                                                );
                                            player.interaction = Some(interaction_entity);
                                        }
                                    } else if let Some(preparation) = table.preparation() {
                                        // PREP TABLE
                                        info!("TRYING TO PREPARE ORDER!");
                                        if let Some(item) = &player.inventory {
                                            // If player has an item in hand, check which.
                                            match item {
                                                Either::Left(flavor) => {
                                                    // Insert scoops into order if possible.
                                                    if table.has_order() {
                                                        let mut order = table.extract_order();
                                                        if order.ingredient_count() > 3 {
                                                            info!(
                                                        "ORDER IS FULL, CANNOT INSERT MORE FLAVORS"
                                                    );
                                                        } else {
                                                            order.insert_flavor(flavor.clone());
                                                            player.inventory = None;
                                                        }
                                                        table.insert_order(order);
                                                    } else {
                                                        table.insert_order(Order::new(
                                                            preparation.clone(),
                                                            flavor.clone(),
                                                        ));
                                                        player.inventory = None;
                                                    }
                                                }
                                                Either::Right(_order) => {
                                                    info!(
                                                        "CANNOT INSERT ORDER BACK INTO PREP TABLE!"
                                                    );
                                                }
                                            }
                                        } else {
                                            info!("PLAYER HAS NOTHING IN INVENTORY!");
                                            if table.has_order() {
                                                let order = table.extract_order();
                                                if order.completed {
                                                    player.inventory = Some(Either::Right(order));
                                                } else if !order.is_empty() {
                                                    info!("IF NOT EMPTY, START MINIGAME!");
                                                    let interaction_entity = self
                                                        .create_preparation_interaction(
                                                            &entities,
                                                            &mut preparation_interactions,
                                                            &mut parents,
                                                            &mut hiddens,
                                                            &mut transparents,
                                                            &mut transforms,
                                                            &mut global_transforms,
                                                            &mut sprites,
                                                            &mut start_pieces,
                                                            &mut middle_pieces,
                                                            &mut end_pieces,
                                                            &mut backgrounds,
                                                            &mut foregrounds,
                                                            &mut sequence_keys,
                                                            &anims,
                                                            player_entity,
                                                            player.palette_key.clone(),
                                                            &player.gamepad_style,
                                                            interact.top,
                                                        );
                                                    player.interaction = Some(interaction_entity);
                                                    table.insert_order(order);
                                                }
                                            }
                                        }
                                    } else if let Some(topping) = table.topping() {
                                        // TOPPING TABLE
                                        info!("TRYING TO INSERT TOPPING!");
                                        if let Some(ref mut item) = &mut player.inventory {
                                            // If player has an item in hand, check which.
                                            match item {
                                                Either::Left(_flavor) => {
                                                    info!("CANNOT INSERT TOPPING ONTO SCOOP!");
                                                }
                                                Either::Right(ref mut order) => {
                                                    // Insert topping if space in order is avalable (3 or
                                                    // less items) and order is completed (prepared).
                                                    if order.completed {
                                                        info!("ORDER IS COMPLETED, CANNOT GO BACK IN!");
                                                        if order.ingredient_count() > 3 {
                                                            info!(
                                                            "ORDER IS FULL, CANNOT INSERT TOPPING"
                                                        );
                                                        } else if order.has_topping() {
                                                            info!("ORDER ALREADY HAS TOPPING");
                                                        } else {
                                                            let interaction_entity = self
                                                                .create_topping_interaction(
                                                                    &entities,
                                                                    &mut topping_interactions,
                                                                    &mut parents,
                                                                    &mut hiddens,
                                                                    &mut transparents,
                                                                    &mut transforms,
                                                                    &mut global_transforms,
                                                                    &mut sprites,
                                                                    &mut start_pieces,
                                                                    &mut middle_pieces,
                                                                    &mut end_pieces,
                                                                    &mut backgrounds,
                                                                    &mut foregrounds,
                                                                    &mut alternative_keys,
                                                                    &anims,
                                                                    topping,
                                                                    player_entity,
                                                                    player.palette_key.clone(),
                                                                    &player.gamepad_style,
                                                                );
                                                            player.interaction =
                                                                Some(interaction_entity);
                                                        }
                                                    } else {
                                                        info!("ORDER IS NOT COMPLETED, ?");
                                                    }
                                                }
                                            }
                                        }
                                    } else if table.delivery() {
                                        // DELIVERY TABLE
                                        info!("TRYING TO DELIVER ORDER!");
                                        if let Some(Either::Right(order)) = &player.inventory {
                                            info!("DELIVERED {:#?}!", order);
                                            player.inventory = None;
                                        } else {
                                            info!("CANNOT DELIVER EMPTY OR SCOOP!");
                                        }
                                    } else {
                                        // Empty Table, can place in/complete orders.
                                    }
                                }
                                input.wants_to_interact = false;
                            }
                            break;
                        }
                    }
                }
            }
        }
    }
}

impl<'s> InteractSystem {
    fn create_flavor_interaction(
        &self,
        entities: &Entities<'s>,
        mut flavor_interactions: &mut WriteStorage<'s, FlavorInteraction>,
        mut parents: &mut WriteStorage<'s, Parent>,
        mut hiddens: &mut WriteStorage<'s, Hidden>,
        mut transparents: &mut WriteStorage<'s, Transparent>,
        mut transforms: &mut WriteStorage<'s, Transform>,
        mut global_transforms: &mut WriteStorage<'s, GlobalTransform>,
        mut sprites: &mut WriteStorage<'s, SpriteRender>,
        mut start_pieces: &mut WriteStorage<'s, StartBarPiece>,
        mut middle_pieces: &mut WriteStorage<'s, MiddleBarPiece>,
        mut end_pieces: &mut WriteStorage<'s, EndBarPiece>,
        mut backgrounds: &mut WriteStorage<'s, BarBackground>,
        mut foregrounds: &mut WriteStorage<'s, BarForeground>,
        mut hold_keys: &mut WriteStorage<'s, HoldKey>,
        anims: &Read<'s, Animations>,
        flavor: FlavorIndex,
        player: Entity,
        key: String,
        style: &Style,
    ) -> Entity {
        let mut entity_transform = Transform::default();
        entity_transform.translation.y = 24.0;
        entity_transform.scale.x = 0.5;
        entity_transform.scale.y = 0.5;

        let interaction = FlavorInteraction::new(2.5, flavor.clone());
        let key_anim = interaction.key.get_str(style);
        let entity = entities
            .build_entity()
            .with(interaction, &mut flavor_interactions)
            .with(Parent { entity: player }, &mut parents)
            .with(entity_transform, &mut transforms)
            .with(GlobalTransform::default(), &mut global_transforms)
            .build();

        self.create_bar(
            &entities,
            &mut parents,
            &mut hiddens,
            &mut transparents,
            &mut transforms,
            &mut global_transforms,
            &mut sprites,
            &mut start_pieces,
            &mut middle_pieces,
            &mut end_pieces,
            &mut backgrounds,
            &mut foregrounds,
            &anims,
            entity,
            key,
        );

        self.create_hold_key(
            entities,
            &mut parents,
            &mut transparents,
            &mut transforms,
            &mut global_transforms,
            &mut sprites,
            &mut hold_keys,
            anims,
            entity,
            key_anim,
        );

        entity
    }

    fn create_preparation_interaction(
        &self,
        entities: &Entities<'s>,
        mut preparation_interactions: &mut WriteStorage<'s, PreparationInteraction>,
        mut parents: &mut WriteStorage<'s, Parent>,
        mut hiddens: &mut WriteStorage<'s, Hidden>,
        mut transparents: &mut WriteStorage<'s, Transparent>,
        mut transforms: &mut WriteStorage<'s, Transform>,
        mut global_transforms: &mut WriteStorage<'s, GlobalTransform>,
        mut sprites: &mut WriteStorage<'s, SpriteRender>,
        mut start_pieces: &mut WriteStorage<'s, StartBarPiece>,
        mut middle_pieces: &mut WriteStorage<'s, MiddleBarPiece>,
        mut end_pieces: &mut WriteStorage<'s, EndBarPiece>,
        mut backgrounds: &mut WriteStorage<'s, BarBackground>,
        mut foregrounds: &mut WriteStorage<'s, BarForeground>,
        mut sequence_keys: &mut WriteStorage<'s, SequenceKey>,
        anims: &Read<'s, Animations>,
        player: Entity,
        key: String,
        style: &Style,
        table: Entity,
    ) -> Entity {
        let mut entity_transform = Transform::default();
        entity_transform.translation.y = 24.0;
        entity_transform.scale.x = 0.5;
        entity_transform.scale.y = 0.5;

        let interaction = PreparationInteraction::new(table, 2);
        let first = interaction.current_key(0, style);
        let second = interaction.current_key(1, style);
        let third = interaction.current_key(2, style);
        let fourth = interaction.current_key(3, style);

        let entity = entities
            .build_entity()
            .with(interaction, &mut preparation_interactions)
            .with(Parent { entity: player }, &mut parents)
            .with(entity_transform, &mut transforms)
            .with(GlobalTransform::default(), &mut global_transforms)
            .build();

        self.create_bar(
            &entities,
            &mut parents,
            &mut hiddens,
            &mut transparents,
            &mut transforms,
            &mut global_transforms,
            &mut sprites,
            &mut start_pieces,
            &mut middle_pieces,
            &mut end_pieces,
            &mut backgrounds,
            &mut foregrounds,
            &anims,
            entity,
            key,
        );

        vec![first, second, third, fourth]
            .iter()
            .enumerate()
            .for_each(|(i, v)| {
                self.create_sequence_key(
                    entities,
                    &mut parents,
                    &mut transparents,
                    &mut transforms,
                    &mut global_transforms,
                    &mut sprites,
                    &mut sequence_keys,
                    anims,
                    entity,
                    v,
                    i,
                )
            });

        entity
    }

    fn create_topping_interaction(
        &self,
        entities: &Entities<'s>,
        mut topping_interactions: &mut WriteStorage<'s, ToppingInteraction>,
        mut parents: &mut WriteStorage<'s, Parent>,
        mut hiddens: &mut WriteStorage<'s, Hidden>,
        mut transparents: &mut WriteStorage<'s, Transparent>,
        mut transforms: &mut WriteStorage<'s, Transform>,
        mut global_transforms: &mut WriteStorage<'s, GlobalTransform>,
        mut sprites: &mut WriteStorage<'s, SpriteRender>,
        mut start_pieces: &mut WriteStorage<'s, StartBarPiece>,
        mut middle_pieces: &mut WriteStorage<'s, MiddleBarPiece>,
        mut end_pieces: &mut WriteStorage<'s, EndBarPiece>,
        mut backgrounds: &mut WriteStorage<'s, BarBackground>,
        mut foregrounds: &mut WriteStorage<'s, BarForeground>,
        mut alternative_keys: &mut WriteStorage<'s, AlternativeKey>,
        anims: &Read<'s, Animations>,
        topping: ToppingIndex,
        player: Entity,
        key: String,
        style: &Style,
    ) -> Entity {
        let mut entity_transform = Transform::default();
        entity_transform.translation.y = 24.0;
        entity_transform.scale.x = 0.5;
        entity_transform.scale.y = 0.5;

        let interaction = ToppingInteraction::new(topping);
        let left = interaction.current_key(false, style);
        let right = interaction.current_key(true, style);

        let entity = entities
            .build_entity()
            .with(interaction, &mut topping_interactions)
            .with(Parent { entity: player }, &mut parents)
            .with(entity_transform, &mut transforms)
            .with(GlobalTransform::default(), &mut global_transforms)
            .build();

        self.create_bar(
            &entities,
            &mut parents,
            &mut hiddens,
            &mut transparents,
            &mut transforms,
            &mut global_transforms,
            &mut sprites,
            &mut start_pieces,
            &mut middle_pieces,
            &mut end_pieces,
            &mut backgrounds,
            &mut foregrounds,
            &anims,
            entity,
            key,
        );

        vec![(false, left), (true, right)]
            .iter()
            .for_each(|(i, v)| {
                self.create_alternative_key(
                    entities,
                    &mut parents,
                    &mut transparents,
                    &mut transforms,
                    &mut global_transforms,
                    &mut sprites,
                    &mut alternative_keys,
                    anims,
                    entity,
                    v,
                    *i,
                )
            });

        entity
    }

    fn create_bar(
        &self,
        entities: &Entities<'s>,
        mut parents: &mut WriteStorage<'s, Parent>,
        mut hiddens: &mut WriteStorage<'s, Hidden>,
        mut transparents: &mut WriteStorage<'s, Transparent>,
        mut transforms: &mut WriteStorage<'s, Transform>,
        mut global_transforms: &mut WriteStorage<'s, GlobalTransform>,
        mut sprites: &mut WriteStorage<'s, SpriteRender>,
        mut start_pieces: &mut WriteStorage<'s, StartBarPiece>,
        mut middle_pieces: &mut WriteStorage<'s, MiddleBarPiece>,
        mut end_pieces: &mut WriteStorage<'s, EndBarPiece>,
        mut backgrounds: &mut WriteStorage<'s, BarBackground>,
        mut foregrounds: &mut WriteStorage<'s, BarForeground>,
        anims: &Read<'s, Animations>,
        entity: Entity,
        key: String,
    ) {
        let mut start_transform = Transform::default();
        start_transform.translation.x = -16.0;

        let mut middle_transform = Transform::default();
        middle_transform.scale.x = 4.0;

        let mut end_transform = Transform::default();
        end_transform.translation.x = 16.0;

        let anim = &anims.animations["base_bar_start"];
        let _start_bg = entities
            .build_entity()
            .with(
                SpriteRender {
                    sprite_sheet: anim.obtain_handle(),
                    sprite_number: anim.get_frame(),
                    flip_horizontal: false,
                    flip_vertical: false,
                },
                &mut sprites,
            )
            .with(Parent { entity }, &mut parents)
            .with(Transparent, &mut transparents)
            .with(BarBackground, &mut backgrounds)
            .with(StartBarPiece, &mut start_pieces)
            .with(start_transform.clone(), &mut transforms)
            .with(GlobalTransform::default(), &mut global_transforms)
            .build();

        let anim = &anims.animations["base_bar_middle"];
        let _middle_bg = entities
            .build_entity()
            .with(
                SpriteRender {
                    sprite_sheet: anim.obtain_handle(),
                    sprite_number: anim.get_frame(),
                    flip_horizontal: false,
                    flip_vertical: false,
                },
                &mut sprites,
            )
            .with(Parent { entity }, &mut parents)
            .with(Transparent, &mut transparents)
            .with(BarBackground, &mut backgrounds)
            .with(MiddleBarPiece, &mut middle_pieces)
            .with(middle_transform.clone(), &mut transforms)
            .with(GlobalTransform::default(), &mut global_transforms)
            .build();

        let anim = &anims.animations["base_bar_end"];
        let _end_bg = entities
            .build_entity()
            .with(
                SpriteRender {
                    sprite_sheet: anim.obtain_handle(),
                    sprite_number: anim.get_frame(),
                    flip_horizontal: false,
                    flip_vertical: false,
                },
                &mut sprites,
            )
            .with(Parent { entity }, &mut parents)
            .with(Transparent, &mut transparents)
            .with(BarBackground, &mut backgrounds)
            .with(EndBarPiece, &mut end_pieces)
            .with(end_transform.clone(), &mut transforms)
            .with(GlobalTransform::default(), &mut global_transforms)
            .build();

        start_transform.translation.z = 1.0;
        middle_transform.translation.z = 1.0;
        end_transform.translation.z = 1.0;

        let anim = &anims.animations[&format!("{}_bar_start", key)];
        let _start_fg = entities
            .build_entity()
            .with(
                SpriteRender {
                    sprite_sheet: anim.obtain_handle(),
                    sprite_number: anim.get_frame(),
                    flip_horizontal: false,
                    flip_vertical: false,
                },
                &mut sprites,
            )
            .with(Parent { entity }, &mut parents)
            .with(Hidden, &mut hiddens)
            .with(Transparent, &mut transparents)
            .with(BarForeground, &mut foregrounds)
            .with(StartBarPiece, &mut start_pieces)
            .with(start_transform, &mut transforms)
            .with(GlobalTransform::default(), &mut global_transforms)
            .build();

        let anim = &anims.animations[&format!("{}_bar_middle", key)];
        let _middle_fg = entities
            .build_entity()
            .with(
                SpriteRender {
                    sprite_sheet: anim.obtain_handle(),
                    sprite_number: anim.get_frame(),
                    flip_horizontal: false,
                    flip_vertical: false,
                },
                &mut sprites,
            )
            .with(Parent { entity }, &mut parents)
            .with(Hidden, &mut hiddens)
            .with(Transparent, &mut transparents)
            .with(BarForeground, &mut foregrounds)
            .with(MiddleBarPiece, &mut middle_pieces)
            .with(middle_transform, &mut transforms)
            .with(GlobalTransform::default(), &mut global_transforms)
            .build();

        let anim = &anims.animations[&format!("{}_bar_end", key)];
        let _end_fg = entities
            .build_entity()
            .with(
                SpriteRender {
                    sprite_sheet: anim.obtain_handle(),
                    sprite_number: anim.get_frame(),
                    flip_horizontal: false,
                    flip_vertical: false,
                },
                &mut sprites,
            )
            .with(Parent { entity }, &mut parents)
            .with(Hidden, &mut hiddens)
            .with(Transparent, &mut transparents)
            .with(BarForeground, &mut foregrounds)
            .with(EndBarPiece, &mut end_pieces)
            .with(end_transform, &mut transforms)
            .with(GlobalTransform::default(), &mut global_transforms)
            .build();
    }

    fn create_hold_key(
        &self,
        entities: &Entities<'s>,
        mut parents: &mut WriteStorage<'s, Parent>,
        mut transparents: &mut WriteStorage<'s, Transparent>,
        mut transforms: &mut WriteStorage<'s, Transform>,
        mut global_transforms: &mut WriteStorage<'s, GlobalTransform>,
        mut sprites: &mut WriteStorage<'s, SpriteRender>,
        mut hold_keys: &mut WriteStorage<'s, HoldKey>,
        anims: &Read<'s, Animations>,
        entity: Entity,
        key: String,
    ) {
        let mut prompt_transform = Transform::default();
        prompt_transform.translation.y = 20.0;

        let anim = &anims.animations[&format!("prompt_{}", key)];
        let _button = entities
            .build_entity()
            .with(
                SpriteRender {
                    sprite_sheet: anim.obtain_handle(),
                    sprite_number: anim.get_frame(),
                    flip_horizontal: false,
                    flip_vertical: false,
                },
                &mut sprites,
            )
            .with(Parent { entity }, &mut parents)
            .with(Transparent, &mut transparents)
            .with(HoldKey, &mut hold_keys)
            .with(prompt_transform, &mut transforms)
            .with(GlobalTransform::default(), &mut global_transforms)
            .build();
    }

    fn create_sequence_key(
        &self,
        entities: &Entities<'s>,
        mut parents: &mut WriteStorage<'s, Parent>,
        mut transparents: &mut WriteStorage<'s, Transparent>,
        mut transforms: &mut WriteStorage<'s, Transform>,
        mut global_transforms: &mut WriteStorage<'s, GlobalTransform>,
        mut sprites: &mut WriteStorage<'s, SpriteRender>,
        mut sequence_keys: &mut WriteStorage<'s, SequenceKey>,
        anims: &Read<'s, Animations>,
        entity: Entity,
        key: &String,
        index: usize,
    ) {
        let mut prompt_transform = Transform::default();
        prompt_transform.translation.x = ((index as isize - 2) as f32 + 0.5) * 20.0;
        prompt_transform.translation.y = 20.0;

        let anim = &anims.animations[&format!("prompt_{}", key)];
        let _button = entities
            .build_entity()
            .with(
                SpriteRender {
                    sprite_sheet: anim.obtain_handle(),
                    sprite_number: anim.get_frame(),
                    flip_horizontal: false,
                    flip_vertical: false,
                },
                &mut sprites,
            )
            .with(Parent { entity }, &mut parents)
            .with(Transparent, &mut transparents)
            .with(SequenceKey(index), &mut sequence_keys)
            .with(prompt_transform, &mut transforms)
            .with(GlobalTransform::default(), &mut global_transforms)
            .build();
    }

    fn create_alternative_key(
        &self,
        entities: &Entities<'s>,
        mut parents: &mut WriteStorage<'s, Parent>,
        mut transparents: &mut WriteStorage<'s, Transparent>,
        mut transforms: &mut WriteStorage<'s, Transform>,
        mut global_transforms: &mut WriteStorage<'s, GlobalTransform>,
        mut sprites: &mut WriteStorage<'s, SpriteRender>,
        mut alternative_keys: &mut WriteStorage<'s, AlternativeKey>,
        anims: &Read<'s, Animations>,
        entity: Entity,
        key: &String,
        side: bool,
    ) {
        let mut prompt_transform = Transform::default();
        prompt_transform.translation.x = if side { -10.0 } else { 10.0 };
        prompt_transform.translation.y = 20.0;

        let anim = &anims.animations[&format!("prompt_{}", key)];
        let _button = entities
            .build_entity()
            .with(
                SpriteRender {
                    sprite_sheet: anim.obtain_handle(),
                    sprite_number: anim.get_frame(),
                    flip_horizontal: false,
                    flip_vertical: false,
                },
                &mut sprites,
            )
            .with(Parent { entity }, &mut parents)
            .with(Transparent, &mut transparents)
            .with(AlternativeKey(side), &mut alternative_keys)
            .with(prompt_transform, &mut transforms)
            .with(GlobalTransform::default(), &mut global_transforms)
            .build();
    }
}
