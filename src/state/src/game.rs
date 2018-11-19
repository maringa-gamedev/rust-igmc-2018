use amethyst::{
    core::{
        cgmath::*,
        transform::{GlobalTransform, Parent, Transform},
    },
    ecs::prelude::*,
    input::{is_close_requested, is_key_down},
    prelude::*,
    renderer::{Camera, Projection, ScreenDimensions, SpriteRender, Transparent, VirtualKeyCode},
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
    map_file: String,
    camera: Option<Entity>,
    entities: Vec<Entity>,
    data: Match,
}

impl Game {
    pub fn with_map(mut self, map_file: &str) -> Self {
        self.map_file = map_file.to_owned();
        self
    }
}

impl Default for Game {
    fn default() -> Self {
        Game {
            map_file: format!("{}/assets/map/0000.ron", application_root_dir()),
            camera: None,
            entities: Vec::with_capacity(128),
            data: Match::default(),
        }
    }
}

const V_W: f32 = 240.0;
const V_H: f32 = 136.0;

impl<'a, 'b> SimpleState<'a, 'b> for Game {
    fn on_start(&mut self, data: StateData<GameData>) {
        let StateData { mut world, .. } = data;

        let flavor_loadout = vec![
            FlavorIndex(0),
            FlavorIndex(0),
            FlavorIndex(1),
            FlavorIndex(1),
            FlavorIndex(2),
            FlavorIndex(2),
            FlavorIndex(7),
            FlavorIndex(7),
        ];

        let preparation_loadout = vec![
            PreparationIndex(0),
            PreparationIndex(0),
            PreparationIndex(0),
            PreparationIndex(0),
        ];
        let topping_loadout = vec![
            ToppingIndex(0),
            ToppingIndex(0),
            ToppingIndex(0),
            ToppingIndex(0),
        ];

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
            &self.map_file,
            &flavor_loadout[..],
            &preparation_loadout[..],
            &topping_loadout[..],
        );
        //self.create_bounds(&mut world, V_W * 2.0, V_H * 2.0);
        self.create_kitchen_bounds(
            &mut world,
            (left_parent, right_parent),
            (0.0, 0.0, BASE * 10.0, BASE * 11.0),
        );

        self.data.flavors = flavor_loadout;
        self.data.preparations = preparation_loadout;
        self.data.toppings = topping_loadout;

        let team_a = Team {
            captain: self.create_player(&mut world, left_parent, spawn_points[0], 0, false),
            server: self.create_player(&mut world, left_parent, spawn_points[1], 1, false),
            scooper_one: None,
            scooper_two: None,
            loadout: vec![],
            power_meter: 0.0,
            score: 0,
            orders: vec![],
        };

        let team_b = Team {
            captain: self.create_player(&mut world, right_parent, spawn_points[0], 2, true),
            server: self.create_player(&mut world, right_parent, spawn_points[1], 3, true),
            scooper_one: None,
            scooper_two: None,
            loadout: vec![],
            power_meter: 0.0,
            score: 0,
            orders: vec![],
        };

        self.data.teams.push(team_a);
        self.data.teams.push(team_b);

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

    fn update(&mut self, data: &mut StateData<GameData>) -> SimpleTrans<'a, 'b> {
        let StateData { world, .. } = data;

        data.data.update(&world);
        //self.update_viewport(&mut world);

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
        parent: Entity,
        (x, y): (f32, f32),
        gamepad_index: usize,
        invert_x_axis: bool,
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
            .with(Player::new(gamepad_index, 0.0, invert_x_axis))
            .with(Input::new())
            .with(Velocity::new(40.0))
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
        item_transform.translation = Vector3::new(0.0, 16.0, -1.0);

        let carry_me = world
            .create_entity()
            .with(SpriteRender {
                sprite_sheet: items_handle,
                sprite_number: 0,
                flip_horizontal: false,
                flip_vertical: false,
            })
            .with(InventoryItem)
            .with(Transparent)
            .with(Layered)
            .with(Parent { entity })
            .with(item_transform)
            .with(GlobalTransform::default())
            .build();
        self.entities.push(carry_me);

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

    fn update_viewport(&mut self, world: &mut World) {
        if let Some(camera) = self.camera.take() {
            world
                .delete_entity(camera)
                .expect("Failed to delete camera entity.");
        }

        let (width, height) = {
            let dim = world.read_resource::<ScreenDimensions>();
            (dim.width(), dim.height())
        };

        let aspect_ratio = V_W / V_H;
        let screen_ratio = width / height;
        let (cam_x, cam_y, cam_w, cam_h) = if screen_ratio < aspect_ratio {
            let has = width / aspect_ratio;
            let y = (height - has) / 2.0;
            (0.0, y, width, has)
        } else if screen_ratio > aspect_ratio {
            let was = height * aspect_ratio;
            let x = (width - was) / 2.0;
            (x, 0.0, was, height)
        } else {
            (0.0, 0.0, width, height)
        };
        info!("Screen: {} x {}", width, height);
        info!("Aspect: {} x {}", cam_w, cam_h);
        info!("Position: {} x {}", cam_x, cam_y);

        let camera_ui = world
            .create_entity()
            .with(Camera::from(Projection::Orthographic(Ortho {
                //left: cam_x,
                left: 0.0,
                right: cam_w,
                top: cam_h,
                //bottom: cam_y,
                bottom: 0.0,
                near: 0.0,
                far: 256.0,
            })))
            .with(GlobalTransform(Matrix4::from_translation(
                Vector3::new(0.0, 0.0, 1.0).into(),
            )))
            .build();
        self.camera = Some(camera_ui);
    }
}
