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

pub fn load_ui_texture(
    world: &mut World,
) -> (SpriteSheetHandle, SpriteSheetHandle, SpriteSheetHandle) {
    info!("Loading ui texture!");

    let data = vec![
        ("texture/other/bg.png", 50, TILE_WIDTH, TILE_HEIGHT),
        ("texture/other/hud.png", 51, MAP_WIDTH, MAP_HEIGHT),
        ("texture/other/title.png", 52, MAP_WIDTH, MAP_HEIGHT),
    ];

    let mut data: Vec<SpriteSheetHandle> = data
        .iter()
        .map(|(path, index, w, h)| {
            let texture = {
                let loader = world.read_resource::<Loader>();
                loader.load(
                    *path,
                    PngFormat,
                    TextureMetadata::srgb_scale(),
                    (),
                    &world.read_resource::<AssetStorage<Texture>>(),
                )
            };
            world
                .write_resource::<MaterialTextureSet>()
                .insert(*index, texture);

            info!("Creating ui sprites.");
            let sprites = vec![Sprite {
                width: *w,
                height: *h,
                offsets: [0.0, 0.0],
                tex_coords: TextureCoordinates {
                    left: 0.0,
                    right: 1.0,
                    top: 1.0,
                    bottom: 0.0,
                },
            }];

            let sprites = SpriteSheet {
                texture_id: *index,
                sprites,
            };

            let handle = {
                let loader = world.read_resource::<Loader>();
                loader.load_from_data(
                    sprites,
                    (),
                    &world.read_resource::<AssetStorage<SpriteSheet>>(),
                )
            };

            world
                .write_resource::<SpriteSheetSet>()
                .insert(*index, handle.clone());

            handle
        })
        .collect();

    (data.remove(0), data.remove(0), data.remove(0))
}

pub fn load_ui_sprites(world: &mut World) -> HashMap<String, Animation> {
    let app_root = application_root_dir();
    let path = format!("{}/assets/texture/other/menu.ron", app_root);
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

    info!("Loading game texture.");
    // Load Textures
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
        .insert(250, texture);

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
                offsets: [sw * w * -0.5, sh * h * -0.5],
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
        texture_id: 250,
        sprites,
    };

    let handle = {
        let loader = world.read_resource::<Loader>();
        loader.load_from_data(
            sprites,
            (),
            &world.read_resource::<AssetStorage<SpriteSheet>>(),
        )
    };

    world
        .write_resource::<SpriteSheetSet>()
        .insert(250, handle.clone());

    let anim_def = anim_def
        .iter()
        .map(|(k, v)| {
            (
                k.to_owned(),
                Animation::new(
                    handle.clone(),
                    v.0.iter()
                        .map(|(name, duration)| (*sprites_hash.get(name).unwrap(), *duration))
                        .collect(),
                    v.1.clone(),
                ),
            )
        })
        .into_iter()
        .collect();

    anim_def
}
