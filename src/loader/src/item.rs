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

pub fn load_items_texture(world: &mut World) -> (SpriteSheetHandle, HashMap<String, Animation>) {
    let app_root = application_root_dir();
    let path = format!("{}/assets/texture/items.ron", app_root);
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

    info!("Loading items texture.");
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
        .insert(ITEMS_TEXTURE_INDEX, texture);

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
                //offsets: [-SPRITE_SIZE / 2.0, -SPRITE_SIZE / 2.0],
                offsets: [
                    0.0,
                    -((SPRITE_HEIGHT / 2.0) - ((PLAYER_HITBOX_HEIGHT / 2.0) + 4.0)),
                ],
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
        texture_id: ITEMS_TEXTURE_INDEX,
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
        .insert(ITEMS_TEXTURE_INDEX, handle.clone());

    let anim_def = anim_def
        .iter()
        .map(|(k, v)| {
            (
                k.to_owned(),
                Animation::new(
                    v.0.iter()
                        .map(|(name, duration)| (*sprites_hash.get(name).unwrap(), *duration))
                        .collect(),
                    v.1.clone(),
                ),
            )
        })
        .into_iter()
        .collect();
    (handle, anim_def)
}
