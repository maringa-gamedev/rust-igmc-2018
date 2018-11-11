#![feature(box_patterns)]
#![feature(exclusive_range_pattern)]

use amethyst::{
    animation::AnimationBundle,
    assets::Processor,
    audio::Source,
    config::*,
    core::transform::TransformBundle,
    input::InputBundle,
    prelude::*,
    renderer::{
        ColorMask, DepthMode, DisplayConfig, DrawSprite, Pipeline, RenderBundle, SpriteRender,
        Stage, ALPHA,
    },
    shrev::*,
    ui::{DrawUi, UiBundle},
    utils::application_root_dir,
};
use gilrs::*;
use log::*;
use std::{collections::HashMap, sync::*};

mod constants;
mod ecs;
mod loader;
mod state;
mod util;

pub struct WorldBounds {
    pub w: f32,
    pub h: f32,
}

impl Default for WorldBounds {
    fn default() -> Self {
        WorldBounds { w: 0.0, h: 0.0 }
    }
}

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let channel = Arc::new(Mutex::new(EventChannel::<ev::Event>::new()));
    {
        let channel = channel.clone();
        std::thread::spawn(move || {
            let mut gilrs = Gilrs::new().unwrap();
            for (_id, gamepad) in gilrs.gamepads() {
                info!(
                    "{} (id:{}) is {:?}",
                    gamepad.os_name(),
                    gamepad.id(),
                    gamepad.power_info()
                );
            }
            loop {
                while let Some(event) = gilrs.next_event() {
                    channel.lock().unwrap().single_write(event);
                }
            }
        });
    }

    let app_root = application_root_dir();
    let path = format!("{}/resources/display.ron", app_root);
    let display_config = DisplayConfig::load(&path);
    let assets_directory = format!("{}/assets/", app_root);
    let key_bindings_path = format!("{}/resources/input.ron", app_root);

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            //21 38 66
            .clear_target([21.0 / 255.0, 38.0 / 255.0, 66.0 / 255.0, 1.0], 1.0)
            .with_pass(DrawSprite::new().with_transparency(
                ColorMask::all(),
                ALPHA,
                Some(DepthMode::LessEqualWrite),
            ))
            .with_pass(DrawUi::new()),
    );
    let game_data = GameDataBuilder::default()
        .with_bundle(AnimationBundle::<u32, SpriteRender>::new(
            "animation_control_system",
            "sampler_interpolation_system",
        ))?
        .with_bundle(
            InputBundle::<String, String>::new().with_bindings_from_file(&key_bindings_path)?,
        )?
        .with(ecs::ControllerSystem::new(), "xto_controller", &[])
        .with(ecs::ControlSystem, "xto_control", &["xto_controller"])
        .with(ecs::InputSystem, "xto_input", &["xto_control"])
        .with(ecs::DirectionSystem, "xto_direction", &["xto_control"])
        .with(ecs::MovementSystem, "xto_movement", &["xto_input"])
        .with(ecs::CollisionSystem, "xto_collision", &["xto_movement"])
        .with(ecs::LayerSystem, "xto_layer", &["xto_collision"])
        .with_bundle(TransformBundle::new().with_dep(&["xto_movement"]))?
        .with_bundle(UiBundle::<String, String>::new())?
        .with(Processor::<Source>::new(), "source_processor", &[])
        .with_bundle(
            RenderBundle::new(pipe, Some(display_config))
                .with_sprite_sheet_processor()
                .with_sprite_visibility_sorting(&["transform_system"]),
        )?;

    let controllers: HashMap<usize, ecs::Controller> = HashMap::new();
    let mut game = Application::build(assets_directory, state::Game::default())?
        .with_resource(Arc::new(Mutex::new(controllers)))
        .with_resource(channel)
        .build(game_data)?;
    game.run();
    Ok(())
}
