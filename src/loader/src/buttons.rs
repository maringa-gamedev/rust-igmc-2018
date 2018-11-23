use amethyst::{
    assets::{AssetStorage, Loader},
    ecs::prelude::*,
    renderer::{
        MaterialTextureSet, PngFormat, Sprite, SpriteSheet, SpriteSheetHandle, SpriteSheetSet,
        Texture, TextureCoordinates, TextureMetadata,
    },
};
use log::*;
use nk_data::*;
use std::collections::HashMap;

pub fn load_interaction_texture(
    world: &mut World,
) -> (
    SpriteSheetHandle,
    SpriteSheetHandle,
    HashMap<String, Animation>,
) {
    // Buttons
    let index = 49;
    let sprites = vec![
        (String::from("prompt_north_left"), 0.0, 0.0, 0.2, 0.5),
        (String::from("prompt_south_left"), 0.2, 0.0, 0.2, 0.5),
        (String::from("prompt_west_left"), 0.4, 0.0, 0.2, 0.5),
        (String::from("prompt_east_left"), 0.6, 0.0, 0.2, 0.5),
        (String::from("prompt_success"), 0.8, 0.0, 0.2, 0.5),
        (String::from("prompt_north_right"), 0.0, 0.5, 0.2, 0.5),
        (String::from("prompt_south_right"), 0.2, 0.5, 0.2, 0.5),
        (String::from("prompt_west_right"), 0.4, 0.5, 0.2, 0.5),
        (String::from("prompt_east_right"), 0.6, 0.5, 0.2, 0.5),
        (String::from("prompt_failure"), 0.8, 0.5, 0.2, 0.5),
    ];
    let texture = {
        let loader = world.read_resource::<Loader>();
        loader.load(
            "texture/ui/buttons.png",
            PngFormat,
            TextureMetadata::srgb_scale(),
            (),
            &world.read_resource::<AssetStorage<Texture>>(),
        )
    };
    world
        .write_resource::<MaterialTextureSet>()
        .insert(index, texture);

    let mut hash = HashMap::new();
    let mut count = 0;
    let sprites = sprites
        .iter()
        .map(|(name, x, y, w, h)| {
            hash.insert(name.clone(), count);
            count += 1;
            Sprite {
                width: 20.0,
                height: 20.0,
                offsets: [0.0, 0.0],
                tex_coords: TextureCoordinates {
                    left: *x,
                    right: x + w,
                    top: y + h,
                    bottom: *y,
                },
            }
        })
        .collect();
    let sprites = SpriteSheet {
        texture_id: index,
        sprites,
    };
    let buttons_handle = {
        let loader = world.read_resource::<Loader>();
        loader.load_from_data(
            sprites,
            (),
            &world.read_resource::<AssetStorage<SpriteSheet>>(),
        )
    };
    world
        .write_resource::<SpriteSheetSet>()
        .insert(index, buttons_handle.clone());
    let buttons_anim: HashMap<String, Animation> = hash
        .iter()
        .map(|(k, v)| {
            (
                k.to_owned(),
                Animation::new(
                    buttons_handle.clone(),
                    vec![(*v, 0.25)],
                    AnimationLoop::Once,
                ),
            )
        })
        .into_iter()
        .collect();

    // Progress
    let index = 48;
    let sprites = vec![
        (String::from("base_bar_start"), 0.0, 0.0, 0.25, 0.1),
        (String::from("base_bar_middle"), 0.25, 0.0, 0.5, 0.1),
        (String::from("base_bar_end"), 0.75, 0.0, 0.25, 0.1),
        (String::from("captain_left_bar_start"), 0.0, 0.1, 0.25, 0.1),
        (String::from("captain_left_bar_middle"), 0.25, 0.1, 0.5, 0.1),
        (String::from("captain_left_bar_end"), 0.75, 0.1, 0.25, 0.1),
        (String::from("server_left_bar_start"), 0.0, 0.2, 0.25, 0.1),
        (String::from("server_left_bar_middle"), 0.25, 0.2, 0.5, 0.1),
        (String::from("server_left_bar_end"), 0.75, 0.2, 0.25, 0.1),
        (
            String::from("scooper_one_left_bar_start"),
            0.0,
            0.3,
            0.25,
            0.1,
        ),
        (
            String::from("scooper_one_left_bar_middle"),
            0.25,
            0.3,
            0.5,
            0.1,
        ),
        (
            String::from("scooper_one_left_bar_end"),
            0.75,
            0.3,
            0.25,
            0.1,
        ),
        (
            String::from("scooper_two_left_bar_start"),
            0.0,
            0.4,
            0.25,
            0.1,
        ),
        (
            String::from("scooper_two_left_bar_middle"),
            0.25,
            0.4,
            0.5,
            0.1,
        ),
        (
            String::from("scooper_two_left_bar_end"),
            0.75,
            0.4,
            0.25,
            0.1,
        ),
        (String::from("captain_right_bar_start"), 0.0, 0.5, 0.25, 0.1),
        (
            String::from("captain_right_bar_middle"),
            0.25,
            0.5,
            0.5,
            0.1,
        ),
        (String::from("captain_right_bar_end"), 0.75, 0.5, 0.25, 0.1),
        (String::from("server_right_bar_start"), 0.0, 0.6, 0.25, 0.1),
        (String::from("server_right_bar_middle"), 0.25, 0.6, 0.5, 0.1),
        (String::from("server_right_bar_end"), 0.75, 0.6, 0.25, 0.1),
        (
            String::from("scooper_one_right_bar_start"),
            0.0,
            0.7,
            0.25,
            0.1,
        ),
        (
            String::from("scooper_one_right_bar_middle"),
            0.25,
            0.7,
            0.5,
            0.1,
        ),
        (
            String::from("scooper_one_right_bar_end"),
            0.75,
            0.7,
            0.25,
            0.1,
        ),
        (
            String::from("scooper_two_right_bar_start"),
            0.0,
            0.8,
            0.25,
            0.1,
        ),
        (
            String::from("scooper_two_right_bar_middle"),
            0.25,
            0.8,
            0.5,
            0.1,
        ),
        (
            String::from("scooper_two_right_bar_end"),
            0.75,
            0.8,
            0.25,
            0.1,
        ),
    ];
    let texture = {
        let loader = world.read_resource::<Loader>();
        loader.load(
            "texture/ui/progress.png",
            PngFormat,
            TextureMetadata::srgb_scale(),
            (),
            &world.read_resource::<AssetStorage<Texture>>(),
        )
    };
    world
        .write_resource::<MaterialTextureSet>()
        .insert(index, texture);
    let mut hash = HashMap::new();
    let mut count = 0;
    let sprites = sprites
        .iter()
        .map(|(name, x, y, w, h)| {
            hash.insert(name.clone(), count);
            count += 1;
            Sprite {
                width: 16.0 * w,
                height: 160.0 * h,
                offsets: [0.0, 0.0],
                tex_coords: TextureCoordinates {
                    left: *x,
                    right: x + w,
                    top: y + h,
                    bottom: *y,
                },
            }
        })
        .collect();
    let sprites = SpriteSheet {
        texture_id: index,
        sprites,
    };
    let progress_handle = {
        let loader = world.read_resource::<Loader>();
        loader.load_from_data(
            sprites,
            (),
            &world.read_resource::<AssetStorage<SpriteSheet>>(),
        )
    };
    world
        .write_resource::<SpriteSheetSet>()
        .insert(index, progress_handle.clone());

    let progress_anim: HashMap<String, Animation> = hash
        .iter()
        .map(|(k, v)| {
            (
                k.to_owned(),
                Animation::new(
                    progress_handle.clone(),
                    vec![(*v, 0.25)],
                    AnimationLoop::Once,
                ),
            )
        })
        .into_iter()
        .collect();

    (
        buttons_handle,
        progress_handle,
        buttons_anim.into_iter().chain(progress_anim).collect(),
    )
}
