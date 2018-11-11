use amethyst::{
    assets::{AssetStorage, Loader},
    ecs::prelude::*,
    renderer::{
        MaterialTextureSet, PngFormat, Sprite, SpriteSheet, SpriteSheetHandle, SpriteSheetSet,
        Texture, TextureCoordinates, TextureMetadata,
    },
    utils::application_root_dir,
};
use crate::constants::*;
use log::*;
use ron::de::from_reader;
use serde_derive::*;
use std::{collections::HashMap, fs::*};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct SpriteCollection {
    pub topping: usize,
    pub delivery: usize,
    pub flavor_a: usize,
    pub flavor_b: usize,
    pub empty: usize,
    pub ground: usize,
    pub preparation: usize,
}

#[derive(Debug)]
pub struct MapSprites {
    pub handle: SpriteSheetHandle,
    pub sprites: HashMap<String, usize>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TextureDefinition {
    path: String,
    width: f32,
    height: f32,
    sprites: Vec<(String, f32, f32, f32, f32)>,
}

pub fn load_map_texture(world: &mut World) -> MapSprites {
    info!("Loading Map Textures.");

    let app_root = application_root_dir();
    let path = format!("{}/assets/texture/map_tiles.ron", app_root);
    let f = File::open(&path).expect("Failed opening file");
    let tex_def: TextureDefinition = match from_reader(f) {
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
        .insert(MAP_INDEX, texture);

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
        texture_id: MAP_INDEX,
        sprites,
    };
    let loader = world.read_resource::<Loader>();
    let handle = loader.load_from_data(
        sprites,
        (),
        &world.read_resource::<AssetStorage<SpriteSheet>>(),
    );
    world
        .write_resource::<SpriteSheetSet>()
        .insert(MAP_INDEX, handle.clone());

    MapSprites {
        handle,
        sprites: sprites_hash,
    }
}
