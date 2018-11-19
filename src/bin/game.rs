#![feature(box_patterns)]
#![feature(exclusive_range_pattern)]
#![feature(type_ascription)]

use amethyst::{
    animation::AnimationBundle,
    assets::Processor,
    audio::Source,
    config::*,
    core::{frame_limiter::FrameRateLimitStrategy, transform::TransformBundle},
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
use clap::{App, Arg};
use gilrs::*;
use log::*;
use nk_ecs::*;
use nk_state::*;
use std::{collections::HashMap, sync::*, time::Duration};

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let matches = App::new("Naisu Kurimu")
        .version("preview-003")
        .author("Tiago Nascimento <xtheosirian@gmail.com>")
        .about("Lançador do Jogo Naisu Kurimu")
        .arg(
            Arg::with_name("map")
                .short("m")
                .long("map")
                .value_name("MAP")
                .help("Specifies which map to open as default")
                .takes_value(true),
        )
        .get_matches();

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
        .with_bundle(GameBundle)?
        .with_bundle(TransformBundle::new().with_dep(&["xto_movement"]))?
        .with_bundle(UiBundle::<String, String>::new())?
        .with(Processor::<Source>::new(), "source_processor", &[])
        .with_bundle(
            RenderBundle::new(pipe, Some(display_config))
                .with_sprite_sheet_processor()
                .with_sprite_visibility_sorting(&["transform_system"]),
        )?;

    let game_state = Load::default();

    let controllers: HashMap<usize, Controller> = HashMap::new();
    let mut game = Application::build(assets_directory, game_state)?
        .with_resource(Arc::new(Mutex::new(controllers)))
        .with_resource(channel)
        .with_resource(matches)
        .with_frame_limit(
            FrameRateLimitStrategy::SleepAndYield(Duration::from_millis(2)),
            30,
        )
        .build(game_data)?;
    game.run();
    Ok(())
}
