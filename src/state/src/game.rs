use super::*;
use amethyst::{
    assets::Loader,
    core::{
        cgmath::*,
        transform::{GlobalTransform, Parent, Transform},
    },
    ecs::prelude::*,
    input::{is_close_requested, is_key_down},
    prelude::*,
    renderer::{
        Camera, Hidden, Projection, ScreenDimensions, SpriteRender, Transparent, VirtualKeyCode,
    },
    ui::{Anchor, FontAsset, FontHandle, TtfFormat, UiFinder, UiText, UiTransform},
    utils::application_root_dir,
};
use either::*;
use log::*;
use nalgebra::Vector2 as NAVector2;
use ncollide2d::shape::*;
use nk_data::*;
use nk_ecs::*;
use nk_loader::*;
use nk_util::*;

#[derive(Debug)]
pub struct Game {
    map: MapDefinition,
    flavor_loadout: Vec<FlavorIndex>,
    preparation_loadout: Vec<PreparationIndex>,
    topping_loadout: Vec<ToppingIndex>,
    camera: Option<Entity>,
    entities: Vec<Entity>,
}

impl Game {
    pub fn with_map(map: MapDefinition) -> Self {
        Game {
            map,
            flavor_loadout: Vec::new(),
            preparation_loadout: Vec::new(),
            topping_loadout: Vec::new(),
            camera: None,
            entities: Vec::with_capacity(128),
        }
    }

    pub fn with_flavors(mut self, flavors: Vec<FlavorIndex>) -> Self {
        self.flavor_loadout = flavors;
        self
    }

    pub fn with_preparations(mut self, preparations: Vec<PreparationIndex>) -> Self {
        self.preparation_loadout = preparations;
        self
    }

    pub fn with_toppings(mut self, toppings: Vec<ToppingIndex>) -> Self {
        self.topping_loadout = toppings;
        self
    }
}

impl<'a, 'b> SimpleState<'a, 'b> for Game {
    fn on_start(&mut self, data: StateData<GameData>) {
        let StateData { mut world, .. } = data;

        let (left_parent, right_parent) = {
            let mut left_transform = Transform::default();
            left_transform.translation = Vector3::new(MAP_OFFSET_X, MAP_OFFSET_Y, 0.0);
            let left = world
                .create_entity()
                .with(left_transform)
                .with(GlobalTransform::default())
                .build();

            let mut right_transform = Transform::default();
            right_transform.translation = Vector3::new(MAP_WIDTH - MAP_OFFSET_X, MAP_OFFSET_Y, 0.0);
            right_transform.scale = Vector3::new(-1.0, 1.0, 1.0);
            let right = world
                .create_entity()
                .with(right_transform)
                .with(GlobalTransform::default())
                .build();

            (left, right)
        };
        self.entities.push(left_parent);
        self.entities.push(right_parent);
        info!("left parent  {:?}", left_parent);
        info!("right parent {:?}", right_parent);

        let (mut map_entities, spawn_points) = create_map_from_file(
            &mut world,
            (left_parent, right_parent),
            &self.map,
            &self.flavor_loadout[..],
            &self.preparation_loadout[..],
            &self.topping_loadout[..],
        );
        //self.create_bounds(&mut world, V_W * 2.0, V_H * 2.0);
        self.create_kitchen_bounds(
            &mut world,
            (left_parent, right_parent),
            (0.0, 0.0, BASE * 10.0, BASE * 11.0),
        );

        let mut data = Match::default();
        data.timer = 5.0 * 60.0;
        data.order_gen_timer = 15.0;

        let team_a = Team {
            captain: self.create_player(
                &mut world,
                0,
                left_parent,
                spawn_points[0],
                0,
                Style::Full,
                false,
                "captain_left",
            ),
            server: self.create_player(
                &mut world,
                0,
                left_parent,
                spawn_points[1],
                1,
                Style::Full,
                false,
                "server_left",
            ),
            scooper_one: None,
            scooper_two: None,
            flavors: self.flavor_loadout.clone(),
            toppings: self.topping_loadout.clone(),
            preparations: self.preparation_loadout.clone(),
            power_meter: 0.0,
            score: 0,
            orders: vec![],
            parent: left_parent,
        };

        let team_b = Team {
            captain: self.create_player(
                &mut world,
                1,
                right_parent,
                spawn_points[0],
                2,
                Style::HalfLeft,
                true,
                "captain_right",
            ),
            server: self.create_player(
                &mut world,
                1,
                right_parent,
                spawn_points[1],
                2,
                Style::HalfRight,
                true,
                "server_right",
            ),
            scooper_one: None,
            scooper_two: None,
            flavors: self.flavor_loadout.clone(),
            toppings: self.topping_loadout.clone(),
            preparations: self.preparation_loadout.clone(),
            power_meter: 0.0,
            score: 0,
            orders: vec![],
            parent: right_parent,
        };

        data.teams.push(team_a);
        data.teams.push(team_b);

        world.add_resource(data);

        self.entities.append(&mut map_entities);

        let mut hud_transform = Transform::default();
        hud_transform.translation = Vector3::new(MAP_WIDTH / 2.0, MAP_HEIGHT / 2.0, 0.0);
        let hud_handle = world.read_resource::<Handles>().hud_handle.clone();
        self.entities.push(
            world
                .create_entity()
                .with(SpriteRender {
                    sprite_sheet: hud_handle,
                    sprite_number: 0,
                    flip_horizontal: false,
                    flip_vertical: false,
                })
                .with(Transparent)
                .with(hud_transform)
                .with(GlobalTransform::default())
                .build(),
        );

        self.create_labels(&mut world);
        self.create_orders_slots(&mut world);
    }

    fn handle_event(
        &mut self,
        _data: StateData<GameData>,
        event: StateEvent,
    ) -> SimpleTrans<'a, 'b> {
        if let StateEvent::Window(event) = &event {
            if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Q) {
                return Trans::Quit;
            }
        }
        Trans::None
    }

    fn update(
        &mut self,
        StateData {
            ref mut world,
            data,
        }: &mut StateData<GameData>,
    ) -> SimpleTrans<'a, 'b> {
        //if let Some(camera) = self.camera.take() {
        //super::update_viewport(camera, world);
        //}

        Trans::None
    }

    fn on_stop(&mut self, data: StateData<GameData>) {
        let StateData { world, .. } = data;
        world
            .delete_entities(self.entities.as_slice())
            .expect("Failed to clean world of Game's entities!");
    }
}

impl Game {
    fn create_player(
        &mut self,
        world: &mut World,
        team_index: usize,
        parent: Entity,
        (x, y): (f32, f32),
        gamepad_index: usize,
        gamepad_style: Style,
        invert_x_axis: bool,
        key: &str,
    ) -> Entity {
        let (player_handle, items_handle) = {
            let handles = world.read_resource::<Handles>();
            (handles.player_handle.clone(), handles.items_handle.clone())
        };

        let mut transform = Transform::default();
        transform.translation = Vector3::new(x, y, 0.0);

        let entity = world
            .create_entity()
            .with(Direction {
                current: Cardinal::South,
                previous: None,
                current_anim: "idle".to_string(),
            })
            .with(SpriteRender {
                sprite_sheet: player_handle,
                sprite_number: 0,
                flip_horizontal: false,
                flip_vertical: false,
            })
            .with(Player::new_with_style(
                gamepad_index,
                gamepad_style,
                team_index,
                0.0,
                invert_x_axis,
                key.to_owned(),
            ))
            .with(Input::new())
            .with(Velocity::new(80.0))
            .with(Hitbox {
                shape: Either::Left(Cuboid::new(NAVector2::new(
                    PLAYER_HITBOX_WIDTH / 2.0,
                    PLAYER_HITBOX_HEIGHT / 2.0,
                ))),
                offset: NAVector2::new(0.0, 0.0),
            })
            .with(Transparent)
            .with(Layered)
            .with(transform)
            .with(Parent { entity: parent })
            .with(GlobalTransform::default())
            .build();
        self.entities.push(entity);

        let mut item_transform = Transform::default();
        item_transform.translation = Vector3::new(0.0, 28.0, 1.0);

        let item_parent = world
            .create_entity()
            .with(Parent { entity })
            .with(item_transform)
            .with(GlobalTransform::default())
            .build();

        (0..6).for_each(|i| {
            let carry_me = world
                .create_entity()
                .with(SpriteRender {
                    sprite_sheet: items_handle.clone(),
                    sprite_number: 0,
                    flip_horizontal: false,
                    flip_vertical: false,
                })
                .with(InventoryItem(i))
                .with(Transparent)
                .with(Hidden)
                .with(Parent {
                    entity: item_parent,
                })
                .with(Transform::default())
                .with(GlobalTransform::default())
                .build();
            self.entities.push(carry_me);
        });

        entity
    }

    fn create_kitchen_bounds(
        &mut self,
        world: &mut World,
        (left_parent, right_parent): (Entity, Entity),
        (x, y, width, height): (f32, f32, f32, f32),
    ) {
        let xs = vec![x + width / 2.0, x + width / 2.0, x - BASE, x + width + BASE];
        let ys = vec![
            y - BASE,
            y + height + BASE,
            y + height / 2.0,
            y + height / 2.0,
        ];
        let ws = vec![
            width + BASE * 2.0,
            width + BASE * 2.0,
            BASE * 2.0,
            BASE * 2.0,
        ];
        let hs = vec![
            BASE * 2.0,
            BASE * 2.0,
            height + BASE * 2.0,
            height + BASE * 2.0,
        ];

        for (((x, y), w), h) in xs.iter().zip(ys).zip(ws).zip(hs) {
            let mut transform = Transform::default();
            transform.translation = Vector3::new(*x, y, 0.0);

            self.entities.push(
                world
                    .create_entity()
                    .with(Solid)
                    .with(Hitbox {
                        shape: Either::Left(Cuboid::new(NAVector2::new(w / 2.0, h / 2.0))),
                        offset: NAVector2::new(0.0, 0.0),
                    })
                    .with(transform.clone())
                    .with(Parent {
                        entity: left_parent,
                    })
                    .with(GlobalTransform::default())
                    .build(),
            );

            self.entities.push(
                world
                    .create_entity()
                    .with(Solid)
                    .with(Hitbox {
                        shape: Either::Left(Cuboid::new(NAVector2::new(w / 2.0, h / 2.0))),
                        offset: NAVector2::new(0.0, 0.0),
                    })
                    .with(transform)
                    .with(Parent {
                        entity: right_parent,
                    })
                    .with(GlobalTransform::default())
                    .build(),
            );
        }
    }

    fn create_bounds(&mut self, world: &mut World, width: f32, height: f32) {
        // Origin center center
        let xs = vec![width / 2.0, width / 2.0, -BASE, width + BASE];
        let ys = vec![-BASE, height + BASE, height / 2.0, height / 2.0];
        let ws = vec![
            width + BASE * 2.0,
            width + BASE * 2.0,
            BASE * 2.0,
            BASE * 2.0,
        ];
        let hs = vec![
            BASE * 2.0,
            BASE * 2.0,
            height + BASE * 2.0,
            height + BASE * 2.0,
        ];

        // Origin bottom left
        //let xs = vec![-BASE, -BASE, -BASE, width];
        //let ys = vec![-BASE, height, 0.0, 0.0];
        //let ws = vec![width + BASE * 2.0, width + BASE * 2.0, BASE, BASE];
        //let hs = vec![BASE, BASE, height, height];

        for (((x, y), w), h) in xs.iter().zip(ys).zip(ws).zip(hs) {
            let mut transform = Transform::default();
            transform.translation = Vector3::new(*x, y, 0.0);

            self.entities.push(
                world
                    .create_entity()
                    .with(Solid)
                    .with(Hitbox {
                        shape: Either::Left(Cuboid::new(NAVector2::new(w / 2.0, h / 2.0))),
                        offset: NAVector2::new(0.0, 0.0),
                    })
                    .with(transform)
                    .with(GlobalTransform::default())
                    .build(),
            );
        }
    }

    fn create_labels(&mut self, world: &mut World) {
        let (score_font, timer_font) = {
            let handles = world.read_resource::<Handles>();
            (handles.score_font.clone(), handles.timer_font.clone())
        };

        // Timer
        let mut transform = Transform::default();
        transform.translation.x = 206.0;
        transform.translation.y = 244.0;
        transform.translation.z = 16.0;

        let timer_parent = world
            .create_entity()
            .with(transform)
            .with(GlobalTransform::default())
            .build();

        // Timer Minutes
        (0..2).for_each(|i| {
            let mut transform = Transform::default();
            transform.translation.x = 17.0 * i as f32;

            world
                .create_entity()
                .with(TimerDigit(i))
                .with(Parent {
                    entity: timer_parent,
                })
                .with(SpriteRender {
                    sprite_sheet: timer_font.clone(),
                    sprite_number: 0,
                    flip_horizontal: false,
                    flip_vertical: false,
                })
                .with(Transparent)
                .with(transform)
                .with(GlobalTransform::default())
                .build();
        });

        // Timer Colon
        let mut transform = Transform::default();
        transform.translation.x = 28.0;

        world
            .create_entity()
            .with(TimerDigit(4))
            .with(Parent {
                entity: timer_parent,
            })
            .with(SpriteRender {
                sprite_sheet: timer_font.clone(),
                sprite_number: 10,
                flip_horizontal: false,
                flip_vertical: false,
            })
            .with(Transparent)
            .with(transform)
            .with(GlobalTransform::default())
            .build();

        // Timer Seconds
        (2..4).for_each(|i| {
            let mut transform = Transform::default();
            transform.translation.x = 6.0 + (17.0 * i as f32);

            world
                .create_entity()
                .with(TimerDigit(i))
                .with(Parent {
                    entity: timer_parent,
                })
                .with(SpriteRender {
                    sprite_sheet: timer_font.clone(),
                    sprite_number: 0,
                    flip_horizontal: false,
                    flip_vertical: false,
                })
                .with(Transparent)
                .with(transform)
                .with(GlobalTransform::default())
                .build();
        });

        // Score Left
        let mut transform = Transform::default();
        transform.translation.x = 5.0;
        transform.translation.y = 248.0;
        transform.translation.z = 16.0;

        let score_left_parent = world
            .create_entity()
            .with(transform)
            .with(GlobalTransform::default())
            .build();

        (0..8).for_each(|i| {
            let mut transform = Transform::default();
            transform.translation.x = 15.0 * i as f32;

            world
                .create_entity()
                .with(ScoreDigit(i, false))
                .with(Parent {
                    entity: score_left_parent,
                })
                .with(SpriteRender {
                    sprite_sheet: score_font.clone(),
                    sprite_number: 0,
                    flip_horizontal: false,
                    flip_vertical: false,
                })
                .with(Transparent)
                .with(transform)
                .with(GlobalTransform::default())
                .build();
        });

        // Score Right
        let mut transform = Transform::default();
        transform.translation.x = 356.0;
        transform.translation.y = 248.0;
        transform.translation.z = 16.0;

        let score_right_parent = world
            .create_entity()
            .with(transform)
            .with(GlobalTransform::default())
            .build();

        (0..8).for_each(|i| {
            let mut transform = Transform::default();
            transform.translation.x = 15.0 * i as f32;

            world
                .create_entity()
                .with(ScoreDigit(i, true))
                .with(Parent {
                    entity: score_right_parent,
                })
                .with(SpriteRender {
                    sprite_sheet: score_font.clone(),
                    sprite_number: 0,
                    flip_horizontal: false,
                    flip_vertical: false,
                })
                .with(Transparent)
                .with(transform)
                .with(GlobalTransform::default())
                .build();
        });
    }

    fn create_orders_slots(&mut self, world: &mut World) {
        let (score_font, timer_font) = {
            let handles = world.read_resource::<Handles>();
            (handles.score_font.clone(), handles.timer_font.clone())
        };

        vec![(0, 50.0), (1, 98.0), (2, 146.0), (3, 194.0)]
            .into_iter()
            .for_each(|(o, y)| {
                let mut transform = Transform::default();
                transform.translation.x = 7.0 + 1.0;
                transform.translation.y = 6.0 + y;
                transform.translation.z = 16.0;

                let parent = world
                    .create_entity()
                    .with(OrderSlot(0, o))
                    .with(transform)
                    .with(GlobalTransform::default())
                    .build();

                vec![
                    (0, 0.0, 0.0),
                    (1, 16.0, 0.0),
                    (2, 0.0, 16.0),
                    (3, 16.0, 16.0),
                ]
                .into_iter()
                .for_each(|(i, x, y)| {
                    let mut transform = Transform::default();
                    transform.translation.x = x;
                    transform.translation.y = y;

                    world
                        .create_entity()
                        .with(OrderIngredient(i))
                        .with(Parent { entity: parent })
                        .with(SpriteRender {
                            sprite_sheet: score_font.clone(),
                            sprite_number: 0,
                            flip_horizontal: false,
                            flip_vertical: false,
                        })
                        //.with(Hidden)
                        .with(Transparent)
                        .with(transform)
                        .with(GlobalTransform::default())
                        .build();
                });
            });

        vec![(0, 50.0), (1, 98.0), (2, 146.0), (3, 194.0)]
            .into_iter()
            .for_each(|(o, y)| {
                let mut transform = Transform::default();
                transform.translation.x = 7.0 + 465.0;
                transform.translation.y = 6.0 + y;
                transform.translation.z = 16.0;
                transform.scale.x = -1.0;

                let parent = world
                    .create_entity()
                    .with(OrderSlot(1, o))
                    .with(transform)
                    .with(GlobalTransform::default())
                    .build();

                vec![
                    (0, 0.0, 0.0),
                    (1, 16.0, 0.0),
                    (2, 0.0, 16.0),
                    (3, 16.0, 16.0),
                ]
                .into_iter()
                .for_each(|(i, x, y)| {
                    let mut transform = Transform::default();
                    transform.translation.x = x;
                    transform.translation.y = y;

                    world
                        .create_entity()
                        .with(OrderIngredient(i))
                        .with(Parent { entity: parent })
                        .with(SpriteRender {
                            sprite_sheet: score_font.clone(),
                            sprite_number: 0,
                            flip_horizontal: false,
                            flip_vertical: false,
                        })
                        //.with(Hidden)
                        .with(Transparent)
                        .with(transform)
                        .with(GlobalTransform::default())
                        .build();
                });
            });
    }
}
