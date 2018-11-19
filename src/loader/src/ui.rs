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
