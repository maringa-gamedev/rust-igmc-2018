use amethyst::{
    core::{
        cgmath::*,
        transform::{GlobalTransform, Transform},
    },
    ecs::prelude::*,
    renderer::{SpriteRender, SpriteSheetHandle},
};
use either::*;
use log::*;
use nalgebra::Vector2 as NAVector2;
use ncollide2d::shape::*;
use nk_data::*;
use nk_ecs::*;
use ron::de::from_reader;
use serde_derive::*;
use std::fs::File;

pub fn create_map_from_file(
    world: &mut World,
    map_handle: SpriteSheetHandle,
    path: &str,
    loadout: &[FlavorIndex],
    definitions: &Definitions,
) -> (Vec<Entity>, Vec<(f32, f32)>) {
    let f = File::open(&path).expect("Failed opening file");
    let map_def: MapDefinition = match from_reader(f) {
        Ok(x) => x,
        Err(e) => {
            error!("Error parsing map file: {}", e);
            panic!("Invalid map file <{}>!", path);
        }
    };

    let count_flavor_tables = map_def
        .tables
        .iter()
        .filter(|t| {
            if let TableType::Flavor(_) = t.2 {
                true
            } else {
                false
            }
        })
        .count();

    if loadout.len() > count_flavor_tables {
        warn!("More flavors than the map has tables!");
    }

    let mut tiles: Vec<Entity> = (0..10)
        .map(|i| {
            (0..11)
                .map(|j| {
                    let mut transform = Transform::default();
                    transform.translation = Vector3::new(
                        (i as f32 + 0.5) * BASE + MAP_OFFSET_X,
                        (j as f32) * BASE + MAP_OFFSET_Y,
                        -1019.0,
                    );
                    world
                        .create_entity()
                        .with(SpriteRender {
                            sprite_sheet: map_handle.clone(),
                            sprite_number: 0,
                            flip_horizontal: false,
                            flip_vertical: false,
                        })
                        .with(AnimatedFloor(String::from("floor")))
                        //.with(Layered)
                        .with(transform)
                        .with(GlobalTransform::default())
                        .build()
                })
                .collect()
        })
        .fold(Vec::with_capacity(10 * 11), |mut acc, v: Vec<Entity>| {
            acc.extend(v);
            acc
        });

    let all: Vec<(Entity, Entity)> = map_def
        .tables
        .iter()
        .map(|(x, y, t, o)| {
            create_table(
                world,
                &map_handle,
                loadout,
                definitions,
                t,
                o,
                *x * BASE + MAP_OFFSET_X,
                *y * BASE + MAP_OFFSET_Y,
            )
        })
        .collect();

    let mut tops: Vec<Entity> = all.iter().map(|(top, _side)| *top).collect();

    let mut sides: Vec<Entity> = all.iter().map(|(_top, side)| *side).collect();

    tiles.append(&mut tops);
    tiles.append(&mut sides);
    (
        tiles,
        map_def
            .spawns
            .iter()
            .map(|(x, y)| (*x * BASE + MAP_OFFSET_X, *y * BASE + MAP_OFFSET_X))
            .collect(),
    )
}

pub fn create_table(
    world: &mut World,
    map_handle: &SpriteSheetHandle,
    loadout: &[FlavorIndex],
    _definitions: &Definitions,
    t: &TableType,
    o: &TableOrientation,
    x: f32,
    y: f32,
) -> (Entity, Entity) {
    let hitbox = |half_w, half_h| Hitbox {
        shape: Either::Left(Cuboid::new(NAVector2::new(
            half_w,
            half_h * o.hitbox_height_multiplier(),
        ))),
        offset: NAVector2::new(0.0, half_h),
    };

    let top = |x, y, half_w, half_h| {
        let mut t = Transform::default();
        t.translation = Vector3::new(x + half_w, y + half_h, 0.0);
        t.set_rotation(Deg(0.0), Deg(0.0), o.make_rot());
        t
    };

    let side = |x, y, half_w, _half_h| {
        let mut t = Transform::default();
        t.translation = Vector3::new(x + half_w, y - (BASE / 4.0), 0.0);
        t.scale = Vector3::new(o.make_scale(), 1.0, 1.0);
        t
    };

    let mut create_both = |hitbox, side, top, key, table| {
        let top = world
            .create_entity()
            .with(SpriteRender {
                sprite_sheet: map_handle.clone(),
                sprite_number: 0,
                flip_horizontal: false,
                flip_vertical: o.flip_vertical(),
            })
            //.with(AnimatedTopTable(key))
            .with(Layered)
            .with(top)
            .with(GlobalTransform::default());
        let top = if let Some(c) = table {
            top.with(c).build()
        } else {
            top.build()
        };
        let side = world
            .create_entity()
            .with(SpriteRender {
                sprite_sheet: map_handle.clone(),
                sprite_number: 0,
                flip_horizontal: false,
                flip_vertical: o.flip_vertical(),
            })
            .with(Solid)
            .with(hitbox)
            .with(Interact {
                highlighted_by: None,
                top,
            })
            .with(AnimatedTable(key))
            .with(Layered)
            .with(side)
            .with(GlobalTransform::default())
            .build();
        (side, top)
    };

    if let TableType::Empty = t {
        let (w, h) = o.make_dim(BASE, BASE);
        let (half_w, half_h) = (w / 2.0, h / 2.0);

        let hitbox = hitbox(half_w, half_h);
        let top = top(x, y, half_w, half_h);
        let mut side = side(x, y, half_w, half_h);
        side.scale = Vector3::new(1.0, 1.0, 1.0);

        create_both(
            hitbox,
            side,
            top,
            String::from("empty"),
            Some(Table::new_empty_table()),
        )
    } else {
        let (w, h) = o.make_dim(BASE * 2.0, BASE);
        let (half_w, half_h) = (w / 2.0, h / 2.0);

        let hitbox = hitbox(half_w, half_h);
        let top = top(x, y, half_w, half_h);
        let side = side(x, y, half_w, half_h);

        match t {
            TableType::Flavor(f) => {
                let table = if let Some(my_flavor) = loadout.get(f.0) {
                    Some(Table::new_flavor_table(my_flavor.clone()))
                } else {
                    None
                };
                create_both(hitbox, side, top, String::from("flavor"), table)
            }
            TableType::Preparation => create_both(
                hitbox,
                side,
                top,
                String::from("preparation"),
                Some(Table::new_preparation_table(PreparationIndex(0))),
            ),
            TableType::Topping => create_both(
                hitbox,
                side,
                top,
                String::from("topping"),
                Some(Table::new_topping_table(ToppingIndex(0))),
            ),
            TableType::Delivery => create_both(
                hitbox,
                side,
                top,
                String::from("delivery"),
                Some(Table::new_delivery_table()),
            ),

            _ => panic!("Impossible!"),
        }
    }
}
