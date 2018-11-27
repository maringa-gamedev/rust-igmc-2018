#![feature(type_ascription)]

use amethyst::{
    core::{cgmath::*, transform::GlobalTransform},
    ecs::prelude::*,
    renderer::{Camera, Projection, ScreenDimensions},
};
use log::*;

mod bundle;
mod game;
mod load;

const V_W: f32 = 240.0;
const V_H: f32 = 136.0;

const TIMER_STR: &str = "timer";

const SCORE_LEFT_STR: &str = "score_left";
const SCORE_RIGHT_STR: &str = "score_right";

const SCORE_WIDTH: f32 = 118.0;
const SCORE_HEIGHT: f32 = 18.0;
const SCORE_SIZE: f32 = 20.0;

const TIMER_WIDTH: f32 = 71.0;
const TIMER_HEIGHT: f32 = 26.0;
const TIMER_SIZE: f32 = 46.0;

const FONT_COLOR: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

pub fn update_viewport(camera: Entity, world: &mut World) -> Entity {
    world
        .delete_entity(camera)
        .expect("Failed to delete camera entity.");

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

    let camera = world
        .create_entity()
        .with(Camera::from(Projection::Orthographic(Ortho {
            left: cam_x,
            right: cam_w,
            top: cam_h,
            bottom: cam_y,
            near: 0.0,
            far: 1152.0,
        })))
        .with(GlobalTransform(Matrix4::from_translation(
            Vector3::new(0.0, 0.0, 128.0).into(),
        )))
        .build();

    camera
}

pub use self::{bundle::*, game::*, load::*};
