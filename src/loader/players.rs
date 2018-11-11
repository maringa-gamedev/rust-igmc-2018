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

pub fn load_players_texture(world: &mut World) -> SpriteSheetHandle {
    info!("Loading game texture.");
    // Load Textures
    let texture = {
        let loader = world.read_resource::<Loader>();
        loader.load(
            "texture/players.png",
            PngFormat,
            TextureMetadata::srgb_scale(),
            (),
            &world.read_resource::<AssetStorage<Texture>>(),
        )
    };
    world
        .write_resource::<MaterialTextureSet>()
        .insert(PLAYERS_TEXTURE_INDEX, texture);

    info!("Creating game sprites.");
    let mut sprites = Vec::with_capacity(32);
    let count_w = 8;
    let count_h = 1;
    let step_w = 1.0 / count_w as f32;
    let step_h = 1.0 / count_h as f32;
    for i in 0..count_h {
        for j in 0..count_w {
            sprites.push(Sprite {
                width: SPRITE_WIDTH,
                height: SPRITE_HEIGHT,
                //offsets: [-SPRITE_SIZE / 2.0, -SPRITE_SIZE / 2.0],
                offsets: [0.0, -((SPRITE_HEIGHT / 2.0) - (PLAYER_HITBOX_HEIGHT / 2.0))],
                tex_coords: TextureCoordinates {
                    left: j as f32 * step_w,
                    right: (j + 1) as f32 * step_w,
                    top: 1.0 - (i + 1) as f32 * step_h,
                    bottom: 1.0 - (i as f32 * step_h),
                },
            });
        }
    }
    let sprites = SpriteSheet {
        texture_id: PLAYERS_TEXTURE_INDEX,
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
        .insert(PLAYERS_TEXTURE_INDEX, handle.clone());

    handle
}
