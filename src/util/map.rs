use amethyst::{
    core::{
        cgmath::*,
        transform::{GlobalTransform, Transform},
    },
    ecs::prelude::*,
    renderer::SpriteRender,
};
use crate::{constants::*, ecs::*, loader::*};
use log::*;
use nalgebra::Vector2 as NAVector2;
use ncollide2d::shape::*;
use ron::de::from_reader;
use serde_derive::*;
use std::fs::File;

#[derive(Debug, Serialize, Deserialize)]
pub enum TableOrientation {
    VerticalLeft,
    VerticalRight,
    HorizontalTop,
    HorizontalBottom,
}

impl Default for TableOrientation {
    fn default() -> Self {
        TableOrientation::HorizontalBottom
    }
}

impl TableOrientation {
    fn flip_vertical(&self) -> bool {
        match self {
            TableOrientation::VerticalLeft => true,
            TableOrientation::VerticalRight => false,
            TableOrientation::HorizontalTop => true,
            TableOrientation::HorizontalBottom => false,
        }
    }

    fn make_dim(&self, w: f32, h: f32) -> (f32, f32) {
        match self {
            TableOrientation::VerticalLeft => (h, w),
            TableOrientation::VerticalRight => (h, w),
            TableOrientation::HorizontalTop => (w, h),
            TableOrientation::HorizontalBottom => (w, h),
        }
    }

    fn make_rot(&self) -> Deg<f32> {
        match self {
            TableOrientation::VerticalLeft => Deg(90.0),
            TableOrientation::VerticalRight => Deg(90.0),
            TableOrientation::HorizontalTop => Deg(0.0),
            TableOrientation::HorizontalBottom => Deg(0.0),
        }
    }

    fn make_scale(&self) -> f32 {
        match self {
            TableOrientation::VerticalLeft => 1.0,
            TableOrientation::VerticalRight => 1.0,
            TableOrientation::HorizontalTop => 2.0,
            TableOrientation::HorizontalBottom => 2.0,
        }
    }

    fn hitbox_height_multiplier(&self) -> f32 {
        match self {
            TableOrientation::VerticalLeft => 0.75,
            TableOrientation::VerticalRight => 0.75,
            TableOrientation::HorizontalTop => 0.5,
            TableOrientation::HorizontalBottom => 0.5,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Table {
    Empty,
    FlavorA,
    FlavorB,
    Preparation,
    Topping,
    Delivery,
}

impl Default for Table {
    fn default() -> Self {
        Table::Empty
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct MapDefinition {
    tables: Vec<(f32, f32, Table, TableOrientation)>,
}

pub fn map_loader(world: &mut World, sprite_def: &MapSprites, path: &str) -> Vec<Entity> {
    let f = File::open(&path).expect("Failed opening file");
    let map_def: MapDefinition = match from_reader(f) {
        Ok(x) => x,
        Err(e) => {
            error!("Error parsing map file: {}", e);
            panic!("Invalid map file <{}>!", path);
        }
    };
    let mut tiles: Vec<Entity> = (0..9)
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
                            sprite_sheet: sprite_def.handle.clone(),
                            sprite_number: sprite_def.top.ground,
                            flip_horizontal: false,
                            flip_vertical: false,
                        })
                        //.with(Layered)
                        .with(transform)
                        .with(GlobalTransform::default())
                        .build()
                })
                .collect()
        })
        .fold(Vec::with_capacity(9 * 11), |mut acc, v: Vec<Entity>| {
            acc.extend(v);
            acc
        });

    let all: Vec<(Entity, Entity)> = map_def
        .tables
        .iter()
        .map(|(x, y, t, o)| {
            create_table(
                world,
                sprite_def,
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
    tiles
}

pub fn create_table(
    world: &mut World,
    sprite_def: &MapSprites,
    t: &Table,
    o: &TableOrientation,
    x: f32,
    y: f32,
) -> (Entity, Entity) {
    let hitbox = |half_w, half_h| Hitbox {
        shape: Cuboid::new(NAVector2::new(
            half_w,
            half_h * o.hitbox_height_multiplier(),
        )),
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

    let mut create_both = |hitbox, side, top, sprite_side, sprite_top, _table_entity| {
        (
            world
                .create_entity()
                .with(SpriteRender {
                    sprite_sheet: sprite_def.handle.clone(),
                    sprite_number: sprite_side,
                    flip_horizontal: false,
                    flip_vertical: o.flip_vertical(),
                })
                .with(Solid)
                .with(hitbox)
                .with(Interaction { margin: 4.0 })
                //.with(table_entity)
                .with(Layered)
                .with(side)
                .with(GlobalTransform::default())
                .build(),
            world
                .create_entity()
                .with(SpriteRender {
                    sprite_sheet: sprite_def.handle.clone(),
                    sprite_number: sprite_top,
                    flip_horizontal: false,
                    flip_vertical: o.flip_vertical(),
                })
                .with(Layered)
                .with(top)
                .with(GlobalTransform::default())
                .build(),
        )
    };

    if let Table::Empty = t {
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
            sprite_def.side.empty,
            sprite_def.top.empty,
            0,
        )
    } else {
        let (w, h) = o.make_dim(BASE * 2.0, BASE);
        let (half_w, half_h) = (w / 2.0, h / 2.0);

        let hitbox = hitbox(half_w, half_h);
        let top = top(x, y, half_w, half_h);
        let side = side(x, y, half_w, half_h);

        match t {
            Table::FlavorA => create_both(
                hitbox,
                side,
                top,
                sprite_def.side.flavor_a,
                sprite_def.top.flavor_a,
                0,
            ),
            Table::FlavorB => create_both(
                hitbox,
                side,
                top,
                sprite_def.side.flavor_b,
                sprite_def.top.flavor_b,
                0,
            ),
            Table::Preparation => create_both(
                hitbox,
                side,
                top,
                sprite_def.side.preparation,
                sprite_def.top.preparation,
                0,
            ),
            Table::Topping => create_both(
                hitbox,
                side,
                top,
                sprite_def.side.topping,
                sprite_def.top.topping,
                0,
            ),
            Table::Delivery => create_both(
                hitbox,
                side,
                top,
                sprite_def.side.delivery,
                sprite_def.top.delivery,
                0,
            ),

            _ => panic!("Impossible!"),
        }
    }
}
