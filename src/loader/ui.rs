use amethyst::{
    assets::{AssetStorage, Loader},
    ecs::prelude::*,
    renderer::{
        MaterialTextureSet, PngFormat, Sprite, SpriteSheet, SpriteSheetHandle, SpriteSheetSet,
        Texture, TextureCoordinates, TextureMetadata,
    },
};
use log::*;

use crate::constants::*;

pub fn load_ui_texture(world: &mut World) -> SpriteSheetHandle {
    info!("Loading ui texture!");
    let texture = {
        let loader = world.read_resource::<Loader>();
        loader.load(
            "texture/tl_back.png",
            PngFormat,
            TextureMetadata::srgb_scale(),
            (),
            &world.read_resource::<AssetStorage<Texture>>(),
        )
    };
    world
        .write_resource::<MaterialTextureSet>()
        .insert(UI_TEXTURE_INDEX, texture);

    info!("Creating ui sprites.");
    let defs = vec![
        (0.0, 0.0, 0.0, 0.0),
        (0.0, 0.0, 0.0, 0.0),
        (0.0, 0.0, 0.0, 0.0),
        (0.0, 0.0, 0.0, 0.0),
        (0.0, 0.0, 0.0, 0.0),
        (0.0, 0.0, 0.0, 0.0),
        (0.0, 0.0, 0.0, 0.0),
        (0.0, 0.0, 0.0, 0.0),
        (0.0, 0.0, 0.0, 0.0),
    ];
    let sprites = vec![Sprite {
        width: TILE_WIDTH,
        height: TILE_HEIGHT,
        offsets: [0.0, 0.0],
        tex_coords: TextureCoordinates {
            left: 0.0,
            right: 1.0,
            top: 0.0,
            bottom: 1.0,
        },
    }];

    let sprites = SpriteSheet {
        texture_id: UI_TEXTURE_INDEX,
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
        .insert(UI_TEXTURE_INDEX, handle.clone());

    handle
}
