use super::game::*;
use amethyst::{
    assets::Loader,
    core::{
        cgmath::*,
        transform::{GlobalTransform, Transform},
    },
    ecs::prelude::*,
    input::{is_close_requested, is_key_down},
    prelude::*,
    renderer::{Camera, Projection, SpriteRender, Transparent, VirtualKeyCode},
    ui::{Anchor, FontAsset, FontHandle, TtfFormat, UiFinder, UiText, UiTransform},
    utils::application_root_dir,
};
use clap::ArgMatches;
use log::*;
use nk_data::*;
use nk_ecs::*;
use nk_loader::*;
use nk_util::*;
use ron::de::from_reader;
use std::{
    collections::HashMap,
    fs::File,
    sync::{Arc, Mutex},
};

#[derive(Debug, Default)]
pub struct Load {
    camera: Option<Entity>,
    entities: Vec<Entity>,
}

impl<'a, 'b> SimpleState<'a, 'b> for Load {
    fn on_start(&mut self, data: StateData<GameData>) {
        let StateData { mut world, .. } = data;

        world.register::<FlavorInteraction>();
        world.register::<PreparationInteraction>();
        world.register::<ToppingInteraction>();
        world.register::<Effect>();

        let camera = world
            .create_entity()
            .with(Camera::from(Projection::Orthographic(Ortho {
                left: 0.0,
                right: VIEW_WIDTH,
                top: VIEW_HEIGHT,
                bottom: 0.0,
                near: 0.0,
                far: 1152.0,
            })))
            .with(GlobalTransform(Matrix4::from_translation(
                Vector3::new(0.0, 0.0, 128.0).into(),
            )))
            .build();
        self.camera = Some(camera);

        initialise_audio(&mut world);
        let (player_handle, mut animations) = load_players_texture(&mut world);
        let (items_handle, items_anims) = load_items_texture(&mut world);
        let (flavors_anims) = load_flavors_texture(&mut world);
        let (toppings_anims) = load_toppings_texture(&mut world);
        let (map_handle, empty_handle, map_anims) = load_map_texture(&mut world);
        let (bg_handle, hud_handle, title_handle) = load_ui_texture(&mut world);
        let (buttons_handle, progress_handle, interaction_anims) =
            load_interaction_texture(&mut world);
        let (score_font, timer_font) = load_number_fonts(&mut world);

        world.add_resource(Handles {
            player_handle,
            items_handle,
            map_handle,
            bg_handle,
            empty_handle,
            hud_handle,
            buttons_handle,
            progress_handle,
            score_font,
            timer_font,
        });

        animations.extend(items_anims);
        animations.extend(flavors_anims);
        animations.extend(toppings_anims);
        animations.extend(map_anims);
        animations.extend(interaction_anims);
        info!("Loaded Animations: {:?}", animations);
        world.add_resource(Animations { animations });

        generate_bg(&mut world);
        info!("Generated background!");

        let palette: HashMap<String, image::Rgba<u8>> = {
            let path = format!("{}/assets/data/palette.ron", application_root_dir());
            let f = File::open(&path).expect("Failed opening 'palette.ron' file!");
            match from_reader(f): Result<[(String, u8, u8, u8, u8); 8], _> {
                Ok(x) => x
                    .iter()
                    .map(|x| (x.0.clone(), image::Rgba([x.2, x.3, x.4, x.1])))
                    .collect(),
                Err(e) => {
                    error!("Error parsing palette definition: {}", e);
                    panic!("Invalid palette definition <{}>!", path);
                }
            }
        };
        info!("Loaded palette: {:?}", palette);
        world.add_resource(Arc::new(Mutex::new(palette)));

        let defs = load_game_data();
        info!("Loaded definitions: {:?}", defs);
        world.add_resource(defs);

        let mut title_transform = Transform::default();
        title_transform.translation = Vector3::new(MAP_WIDTH / 2.0, MAP_HEIGHT / 2.0, 0.0);
        let title_card = world
            .create_entity()
            .with(SpriteRender {
                sprite_sheet: title_handle,
                sprite_number: 0,
                flip_horizontal: false,
                flip_vertical: false,
            })
            .with(Transparent)
            .with(title_transform)
            .with(GlobalTransform::default())
            .build();
        self.entities.push(title_card);
    }

    fn handle_event(
        &mut self,
        data: StateData<GameData>,
        event: StateEvent,
    ) -> SimpleTrans<'a, 'b> {
        let StateData { world, .. } = data;

        if let StateEvent::Window(event) = &event {
            if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Q) {
                return Trans::Quit;
            } else if is_key_down(&event, VirtualKeyCode::Return) {
                let matches = world.read_resource::<ArgMatches>();
                return Trans::Switch(Box::new(Game::default().with_map(&format!(
                    "{}/assets/map/{}.ron",
                    application_root_dir(),
                    matches.value_of("map").unwrap_or("0000")
                ))));
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
            .expect("Failed to clean world of Load's entities!");
    }
}
