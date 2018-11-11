use amethyst::{
    assets::{AssetStorage, Loader},
    ecs::prelude::*,
    renderer::{
        MaterialTextureSet, PngFormat, Sprite, SpriteSheet, SpriteSheetHandle, SpriteSheetSet,
        Texture, TextureCoordinates, TextureMetadata,
    },
};
use crate::constants::*;
use log::*;
use serde_derive::*;

const HALF: f32 = 0.5;
const QUARTER: f32 = 0.25;
const HALF_QUARTER: f32 = 0.125;

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
    pub top: SpriteCollection,
    pub side: SpriteCollection,
}

pub fn load_map_texture(world: &mut World) -> MapSprites {
    info!("Creating map sprites.");
    let map_handle = {
        let texture = {
            let loader = world.read_resource::<Loader>();
            loader.load(
                "texture/map_tiles.png",
                PngFormat,
                TextureMetadata::srgb_scale(),
                (),
                &world.read_resource::<AssetStorage<Texture>>(),
            )
        };
        world
            .write_resource::<MaterialTextureSet>()
            .insert(MAP_INDEX, texture);

        let (sw, sh) = (BASE * 4.0, BASE * 4.0);
        let defs = vec![
            (0.0, 0.0, HALF, QUARTER),                                   // Topping
            (HALF, 0.0, HALF, QUARTER),                                  // Delivery
            (0.0, QUARTER, HALF, QUARTER),                               // FlavorA
            (HALF, QUARTER, HALF, QUARTER),                              // FlavorB
            (0.0, HALF, QUARTER, QUARTER),                               // Empty
            (QUARTER, HALF, QUARTER, QUARTER),                           // Ground
            (HALF, HALF, HALF, QUARTER),                                 // Preparation
            (0.0, HALF + QUARTER, QUARTER, HALF_QUARTER),                // Topping
            (QUARTER, HALF + QUARTER, QUARTER, HALF_QUARTER),            // Delivery
            (HALF, HALF + QUARTER, QUARTER, HALF_QUARTER),               // FlavorA
            (HALF + QUARTER, HALF + QUARTER, QUARTER, HALF_QUARTER),     // FlavorB
            (0.0, HALF + QUARTER + HALF_QUARTER, QUARTER, HALF_QUARTER), // Empty
            (
                QUARTER,
                HALF + QUARTER + HALF_QUARTER,
                QUARTER,
                HALF_QUARTER,
            ), // Ground
            (HALF, HALF + QUARTER + HALF_QUARTER, QUARTER, HALF_QUARTER), // Preparation
        ];
        let sprites = defs
            .iter()
            .map(|(x, y, w, h)| Sprite {
                width: sw * w,
                height: sh * h,
                offsets: [0.0, 0.0],
                tex_coords: TextureCoordinates {
                    left: *x,
                    right: x + w,
                    top: y + h,
                    bottom: *y,
                },
            })
            .collect();
        let sprites = SpriteSheet {
            texture_id: MAP_INDEX,
            sprites,
        };
        let loader = world.read_resource::<Loader>();
        loader.load_from_data(
            sprites,
            (),
            &world.read_resource::<AssetStorage<SpriteSheet>>(),
        )
    };
    world
        .write_resource::<SpriteSheetSet>()
        .insert(MAP_INDEX, map_handle.clone());

    MapSprites {
        handle: map_handle,
        top: SpriteCollection {
            topping: 0,
            delivery: 1,
            flavor_a: 2,
            flavor_b: 3,
            empty: 4,
            ground: 5,
            preparation: 6,
        },
        side: SpriteCollection {
            topping: 7,
            delivery: 8,
            flavor_a: 9,
            flavor_b: 10,
            empty: 11,
            ground: 12,
            preparation: 13,
        },
    }
}
