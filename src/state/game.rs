use amethyst::{
    core::{
        cgmath::*,
        transform::{GlobalTransform, Transform},
    },
    ecs::prelude::*,
    input::{is_close_requested, is_key_down},
    prelude::*,
    renderer::{
        Camera, Projection, ScreenDimensions, SpriteRender, SpriteSheetHandle, VirtualKeyCode,
    },
    utils::application_root_dir,
};
use log::*;
use nalgebra::Vector2 as NAVector2;
use ncollide2d::shape::*;

use crate::{constants::*, ecs::*, util::*, WorldBounds};

#[derive(Debug, Default)]
pub struct Game {
    camera: Option<Entity>,
    entities: Vec<Entity>,
    player_one: Option<Entity>,
    player_two: Option<Entity>,
    opponent: Option<Entity>,
}

const V_W: f32 = 240.0;
const V_H: f32 = 136.0;

impl<'a, 'b> SimpleState<'a, 'b> for Game {
    fn on_start(&mut self, data: StateData<GameData>) {
        let StateData { mut world, .. } = data;

        world.register::<Interaction>();
        world.register::<Effect>();

        // 480.0 x 272.0
        world.add_resource(WorldBounds {
            w: V_W * 2.0,
            h: V_H * 2.0,
        });

        let camera_ui = world
            .create_entity()
            .with(Camera::from(Projection::Orthographic(Ortho {
                left: 0.0,
                right: V_W * 2.0,
                top: V_H * 2.0,
                bottom: 0.0,
                near: 0.0,
                far: 1024.0,
            })))
            .with(GlobalTransform(Matrix4::from_translation(
                Vector3::new(0.0, 0.0, 1.0).into(),
            )))
            .build();

        self.camera = Some(camera_ui);

        let ui_texture_handle = crate::loader::load_ui_texture(&mut world);
        let players_texture_handle = crate::loader::load_players_texture(&mut world);
        let map_sprites = crate::loader::load_map_texture(&mut world);

        let width_count = ((V_W * 2.0) / 16.0) as isize;
        let height_count = ((V_H * 2.0) / 16.0) as isize;
        for i in 0..width_count {
            for j in 0..height_count {
                let mut transform = Transform::default();
                transform.translation =
                    Vector3::new(8.0 + (i as f32 * 16.0), 8.0 + (j as f32 * 16.0), -1020.0);

                let entity = world
                    .create_entity()
                    .with(SpriteRender {
                        sprite_sheet: ui_texture_handle.clone(),
                        sprite_number: 0,
                        flip_horizontal: false,
                        flip_vertical: false,
                    })
                    .with(transform)
                    .with(GlobalTransform::default())
                    .build();
                self.entities.push(entity);
            }
        }

        //let center = (V_W, V_H);
        self.create_player(
            &mut world,
            177.0,
            144.0,
            0,
            players_texture_handle.clone(),
            0,
            Deg(0.0),
        );
        self.create_bounds(&mut world, V_W * 2.0, V_H * 2.0);

        let app_root = application_root_dir();
        let map_path = format!("{}/assets/map/0001.ron", app_root);
        let mut map_entities = map_loader(&mut world, &map_sprites, &map_path);
        self.entities.append(&mut map_entities);
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
            .expect("Failed to clean world of Games' entities!");
    }
}

impl Game {
    fn create_player(
        &mut self,
        world: &mut World,
        x: f32,
        y: f32,
        gamepad_index: usize,
        handle: SpriteSheetHandle,
        index: usize,
        angle: Deg<f32>,
    ) {
        let mut transform = Transform::default();
        transform.translation = Vector3::new(x, y, 0.0);
        transform.set_rotation(Deg::zero(), Deg::zero(), angle);

        let entity = world
            .create_entity()
            .with(Direction {
                current: Cardinal::North,
                previous: None,
                north: 0,
                north_east: 1,
                east: 2,
                south_east: 3,
                south: 4,
                south_west: 5,
                west: 6,
                north_west: 7,
            })
            .with(PixelPerfect {
                position: Vector2::new(x, y),
            })
            .with(SpriteRender {
                sprite_sheet: handle,
                sprite_number: index,
                flip_horizontal: false,
                flip_vertical: true,
            })
            .with(Player::new(gamepad_index, 0.0))
            .with(Input::new(Vector2::new(angle.cos(), angle.sin())))
            .with(Velocity::new(20.0))
            .with(Hitbox {
                shape: Cuboid::new(NAVector2::new(
                    PLAYER_HITBOX_WIDTH / 2.0,
                    PLAYER_HITBOX_HEIGHT / 2.0,
                )),
                offset: NAVector2::new(0.0, 0.0),
            })
            .with(Layered)
            .with(transform)
            .with(GlobalTransform::default())
            .build();
        self.entities.push(entity);
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

            let entity = world
                .create_entity()
                .with(Solid)
                .with(Hitbox {
                    shape: Cuboid::new(NAVector2::new(w / 2.0, h / 2.0)),
                    offset: NAVector2::new(0.0, 0.0),
                })
                .with(transform)
                .with(GlobalTransform::default())
                .build();
            self.entities.push(entity);
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
