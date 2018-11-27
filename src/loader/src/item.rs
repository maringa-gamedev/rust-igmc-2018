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
    let path = format!("{}/assets/texture/item/old-index.ron", app_root);
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
                offsets: [0.0, 0.0],
                //offsets: [-SPRITE_SIZE / 2.0, -SPRITE_SIZE / 2.0],
                /*
                 *offsets: [
                 *    0.0,
                 *    -((SPRITE_HEIGHT / 2.0) - ((PLAYER_HITBOX_HEIGHT / 2.0) + 4.0)),
                 *],
                 */
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

    (handle, anim_def)
}

pub fn load_flavors_texture(world: &mut World) -> (HashMap<String, Animation>) {
    let app_root = application_root_dir();
    let path = format!("{}/assets/texture/item/flavors.ron", app_root);
    let f = File::open(&path).expect("Failed opening file");
    let sprites_folder: SpriteFolderDef = match from_reader(f) {
        Ok(x) => x,
        Err(e) => {
            error!("Error parsing sprite folder definition: {}", e);
            panic!("Invalid sprite folder definition <{}>!", path);
        }
    };

    let mut tindex = 300;
    let sprites = sprites_folder
        .keys
        .iter()
        .map(|key| {
            let texture = {
                let loader = world.read_resource::<Loader>();
                loader.load(
                    format!("{}/assets/texture/item/{}.png", app_root, key),
                    PngFormat,
                    TextureMetadata::srgb_scale(),
                    (),
                    &world.read_resource::<AssetStorage<Texture>>(),
                )
            };
            world
                .write_resource::<MaterialTextureSet>()
                .insert(tindex, texture);

            let mut sprites_hash = HashMap::new();
            let handle = {
                let (sw, sh) = (sprites_folder.width, sprites_folder.height);
                let mut index: usize = 0;
                world.read_resource::<Loader>().load_from_data(
                    SpriteSheet {
                        texture_id: tindex,
                        sprites: sprites_folder
                            .sprites
                            .iter()
                            .map(|slice| {
                                let (name, x, y, w, h) =
                                    (&slice.0, &slice.1, &slice.2, &slice.3, &slice.4);
                                sprites_hash.insert(format!("{}_{}", key, name), index);
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
                            .collect(),
                    },
                    (),
                    &world.read_resource::<AssetStorage<SpriteSheet>>(),
                )
            };
            world
                .write_resource::<SpriteSheetSet>()
                .insert(tindex, handle.clone());

            tindex += 1;

            let anims: HashMap<String, Animation> = sprites_folder
                .anims
                .iter()
                .map(|(aname, def)| {
                    let (frames, loop_type) = (&def.0, &def.1);
                    (
                        format!("{}_{}", key, aname),
                        Animation::new(
                            handle.clone(),
                            frames
                                .iter()
                                .map(|frame| {
                                    let (fname, duration) = (&frame.0, &frame.1);
                                    (
                                        *sprites_hash.get(&format!("{}_{}", key, fname)).unwrap(),
                                        *duration,
                                    )
                                })
                                .collect(),
                            loop_type.clone(),
                        ),
                    )
                })
                .collect();
            anims
        })
        .fold(HashMap::new(), |mut acc, v| {
            acc.extend(v);
            acc
        });

    sprites
}

pub fn load_toppings_texture(world: &mut World) -> (HashMap<String, Animation>) {
    let app_root = application_root_dir();
    let path = format!("{}/assets/texture/item/toppings.ron", app_root);
    let f = File::open(&path).expect("Failed opening file");
    let sprites_folder: SpriteFolderDef = match from_reader(f) {
        Ok(x) => x,
        Err(e) => {
            error!("Error parsing sprite folder definition: {}", e);
            panic!("Invalid sprite folder definition <{}>!", path);
        }
    };

    let mut tindex = 400;
    let sprites = sprites_folder
        .keys
        .iter()
        .map(|key| {
            let texture = {
                let loader = world.read_resource::<Loader>();
                loader.load(
                    format!("{}/assets/texture/item/{}.png", app_root, key),
                    PngFormat,
                    TextureMetadata::srgb_scale(),
                    (),
                    &world.read_resource::<AssetStorage<Texture>>(),
                )
            };
            world
                .write_resource::<MaterialTextureSet>()
                .insert(tindex, texture);

            let mut sprites_hash = HashMap::new();
            let handle = {
                let (sw, sh) = (sprites_folder.width, sprites_folder.height);
                let mut index: usize = 0;
                world.read_resource::<Loader>().load_from_data(
                    SpriteSheet {
                        texture_id: tindex,
                        sprites: sprites_folder
                            .sprites
                            .iter()
                            .map(|slice| {
                                let (name, x, y, w, h) =
                                    (&slice.0, &slice.1, &slice.2, &slice.3, &slice.4);
                                sprites_hash.insert(format!("{}_{}", key, name), index);
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
                            .collect(),
                    },
                    (),
                    &world.read_resource::<AssetStorage<SpriteSheet>>(),
                )
            };
            world
                .write_resource::<SpriteSheetSet>()
                .insert(tindex, handle.clone());

            tindex += 1;

            let anims: HashMap<String, Animation> = sprites_folder
                .anims
                .iter()
                .map(|(aname, def)| {
                    let (frames, loop_type) = (&def.0, &def.1);
                    (
                        format!("{}_{}", key, aname),
                        Animation::new(
                            handle.clone(),
                            frames
                                .iter()
                                .map(|frame| {
                                    let (fname, duration) = (&frame.0, &frame.1);
                                    (
                                        *sprites_hash.get(&format!("{}_{}", key, fname)).unwrap(),
                                        *duration,
                                    )
                                })
                                .collect(),
                            loop_type.clone(),
                        ),
                    )
                })
                .collect();
            anims
        })
        .fold(HashMap::new(), |mut acc, v| {
            acc.extend(v);
            acc
        });

    sprites
}
