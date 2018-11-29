use amethyst::{
    assets::{AssetStorage, Loader},
    ecs::prelude::*,
    renderer::{
        MaterialTextureSet, PngFormat, Sprite, SpriteSheet, SpriteSheetHandle, SpriteSheetSet,
        Texture, TextureCoordinates, TextureMetadata,
    },
    utils::application_root_dir,
};
use log::*;
use nk_data::*;
use ron::de::from_reader;
use std::{collections::HashMap, fs::File};

pub fn load_interaction_texture(
    world: &mut World,
) -> (
    SpriteSheetHandle,
    SpriteSheetHandle,
    HashMap<String, Animation>,
) {
    // Buttons
    let tindex = 49;
    let app_root = application_root_dir();
    let path = format!("{}/assets/texture/ui/buttons.ron", app_root);
    let f = File::open(&path).expect("Failed opening file");
    let (tex_def, anim_def): (
        TextureDefinition,
        HashMap<String, (Vec<(String, f32)>, AnimationLoop)>,
    ) = match from_reader(f) {
        Ok(x) => x,
        Err(e) => {
            error!("Error parsing texture definition: {}", e);
            panic!("Invalid texture definition <{}>!", path);
        }
    };
    let texture = {
        let loader = world.read_resource::<Loader>();
        loader.load(
            tex_def.path,
            PngFormat,
            TextureMetadata::srgb_scale(),
            (),
            &world.read_resource::<AssetStorage<Texture>>(),
        )
    };
    world
        .write_resource::<MaterialTextureSet>()
        .insert(tindex, texture);
    let (sw, sh) = (tex_def.width, tex_def.height);
    let mut sprites_hash = HashMap::new();
    let mut index: usize = 0;
    let sprites = tex_def
        .sprites
        .iter()
        .map(|(name, x, y, w, h)| {
            sprites_hash.insert(name.clone(), index);
            index += 1;
            Sprite {
                width: sw * w,
                height: sh * h,
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
        texture_id: tindex,
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
        .insert(tindex, buttons_handle.clone());

    let buttons_anim: HashMap<String, Animation> = anim_def
        .iter()
        .map(|(k, v)| {
            (
                k.to_owned(),
                Animation::new(
                    buttons_handle.clone(),
                    v.0.iter()
                        .map(|(name, duration)| (*sprites_hash.get(name).unwrap(), *duration))
                        .collect(),
                    v.1.clone(),
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
