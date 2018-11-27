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

pub fn load_number_fonts(world: &mut World) -> (SpriteSheetHandle, SpriteSheetHandle) {
    // Buttons
    let index = 30;
    let sprites = vec![
        (0.0, 0.5, 0.1, 0.5),
        (0.1, 0.5, 0.1, 0.5),
        (0.2, 0.5, 0.1, 0.5),
        (0.3, 0.5, 0.1, 0.5),
        (0.4, 0.5, 0.1, 0.5),
        (0.5, 0.5, 0.1, 0.5),
        (0.6, 0.5, 0.1, 0.5),
        (0.7, 0.5, 0.1, 0.5),
        (0.8, 0.5, 0.1, 0.5),
        (0.9, 0.5, 0.1, 0.5),
        (0.0, 0.0, 0.1, 0.5),
    ];
    let texture = {
        let loader = world.read_resource::<Loader>();
        loader.load(
            "texture/block-numbers.png",
            PngFormat,
            TextureMetadata::srgb_scale(),
            (),
            &world.read_resource::<AssetStorage<Texture>>(),
        )
    };
    world
        .write_resource::<MaterialTextureSet>()
        .insert(index, texture);

    let sprites = sprites
        .iter()
        .map(|(x, y, w, h)| Sprite {
            width: 13.0,
            height: 18.0,
            offsets: [-6.5, -9.0],
            tex_coords: TextureCoordinates {
                left: *x,
                right: x + w,
                top: y + h,
                bottom: *y,
            },
        })
        .collect();
    let sprites = SpriteSheet {
        texture_id: index,
        sprites,
    };
    let score_font = {
        let loader = world.read_resource::<Loader>();
        loader.load_from_data(
            sprites,
            (),
            &world.read_resource::<AssetStorage<SpriteSheet>>(),
        )
    };
    world
        .write_resource::<SpriteSheetSet>()
        .insert(index, score_font.clone());

    // Progress
    let index = 31;
    let sprites = vec![
        (0.0, 0.5, 0.1, 0.5),
        (0.1, 0.5, 0.1, 0.5),
        (0.2, 0.5, 0.1, 0.5),
        (0.3, 0.5, 0.1, 0.5),
        (0.4, 0.5, 0.1, 0.5),
        (0.5, 0.5, 0.1, 0.5),
        (0.6, 0.5, 0.1, 0.5),
        (0.7, 0.5, 0.1, 0.5),
        (0.8, 0.5, 0.1, 0.5),
        (0.9, 0.5, 0.1, 0.5),
        (0.0, 0.0, 0.1, 0.5),
    ];
    let texture = {
        let loader = world.read_resource::<Loader>();
        loader.load(
            "texture/tall-numbers.png",
            PngFormat,
            TextureMetadata::srgb_scale(),
            (),
            &world.read_resource::<AssetStorage<Texture>>(),
        )
    };
    world
        .write_resource::<MaterialTextureSet>()
        .insert(index, texture);
    let sprites = sprites
        .iter()
        .map(|(x, y, w, h)| Sprite {
            width: 14.0,
            height: 26.0,
            offsets: [-7.0, -13.0],
            tex_coords: TextureCoordinates {
                left: *x,
                right: x + w,
                top: y + h,
                bottom: *y,
            },
        })
        .collect();
    let sprites = SpriteSheet {
        texture_id: index,
        sprites,
    };
    let timer_font = {
        let loader = world.read_resource::<Loader>();
        loader.load_from_data(
            sprites,
            (),
            &world.read_resource::<AssetStorage<SpriteSheet>>(),
        )
    };
    world
        .write_resource::<SpriteSheetSet>()
        .insert(index, timer_font.clone());

    (score_font, timer_font)
}
