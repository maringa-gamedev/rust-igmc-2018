use amethyst::{
    core::{
        cgmath::*,
        transform::{GlobalTransform, Parent, Transform},
    },
    ecs::prelude::*,
    renderer::{SpriteRender, SpriteSheetHandle},
    utils::application_root_dir,
};
use either::*;
use log::*;
use nalgebra::Vector2 as NAVector2;
use ncollide2d::shape::*;
use nk_data::*;
use nk_ecs::*;
use ron::de::from_reader;
use std::fs::File;

pub fn load_freeplay_defs() -> Vec<MapDefinition> {
    let app_root = application_root_dir();
    let path = format!("{}/assets/map/freeplay.ron", app_root);
    let f = File::open(&path).expect("Failed opening file");
    let files: Vec<String> = match from_reader(f) {
        Ok(x) => x,
        Err(e) => {
            error!("Error parsing freeplay collection file: {}", e);
            return Vec::new();
        }
    };

    files
        .into_iter()
        .filter_map(|f| {
            let f = File::open(format!("{}/assets/map/{}.ron", app_root, f))
                .expect("Failed opening file");
            let map_def: MapDefinition = match from_reader(f) {
                Ok(x) => x,
                Err(e) => {
                    error!("Error parsing map file: {}", e);
                    return None;
                }
            };

            Some(map_def)
        })
        .collect()
}

pub fn create_map_preview(world: &mut World, map: &MapDefinition) -> Entity {
    let (
        map_preview_handle,
        map_preview_floor,
        map_preview_flavor,
        map_preview_preparation,
        map_preview_topping,
        map_preview_delivery,
        map_preview_empty,
    ) = {
        let anims = world.read_resource::<Animations>();
        let anims = &anims.animations;
        (
            anims["map_preview_floor"].obtain_handle(),
            anims["map_preview_floor"].get_frame(),
            anims["map_preview_flavor"].get_frame(),
            anims["map_preview_preparation"].get_frame(),
            anims["map_preview_topping"].get_frame(),
            anims["map_preview_delivery"].get_frame(),
            anims["map_preview_empty"].get_frame(),
        )
    };

    let mut transform = Transform::default();
    transform.translation = Vector3::new(0.0, 0.0, 1.0);

    let parent = world
        .create_entity()
        .with(AnimatedFloor(String::from("floor")))
        .with(transform)
        .with(GlobalTransform::default())
        .build();

    (0..10).for_each(|i| {
        (0..11).for_each(|j| {
            let mut transform = Transform::default();
            transform.translation = Vector3::new(i as f32 * 8.0, j as f32 * 8.0, -1.0);
            world
                .create_entity()
                .with(SpriteRender {
                    sprite_sheet: map_preview_handle.clone(),
                    sprite_number: map_preview_floor,
                    flip_horizontal: false,
                    flip_vertical: false,
                })
                .with(transform)
                .with(GlobalTransform::default())
                .with(Parent { entity: parent })
                .build();
        });
    });

    map.tables.iter().for_each(|(x, y, t, o)| {
        let mut transform = Transform::default();
        transform.translation = Vector3::new(x * 8.0, y * 8.0, 0.0);
        let frame = match t {
            TableType::Flavor(_) => map_preview_flavor,
            TableType::Preparation(_) => map_preview_preparation,
            TableType::Topping(_) => map_preview_topping,
            TableType::Delivery => map_preview_delivery,
            TableType::Empty => map_preview_empty,
        };

        world
            .create_entity()
            .with(SpriteRender {
                sprite_sheet: map_preview_handle.clone(),
                sprite_number: frame,
                flip_horizontal: false,
                flip_vertical: false,
            })
            .with(transform)
            .with(GlobalTransform::default())
            .with(Parent { entity: parent })
            .build();

        match t {
            TableType::Empty => {}
            _ => match o {
                TableOrientation::VerticalRight | TableOrientation::VerticalLeft => {
                    let mut transform = Transform::default();
                    transform.translation = Vector3::new(x * 8.0, (y + 1.0) * 8.0, 0.0);

                    world
                        .create_entity()
                        .with(SpriteRender {
                            sprite_sheet: map_preview_handle.clone(),
                            sprite_number: frame,
                            flip_horizontal: false,
                            flip_vertical: false,
                        })
                        .with(transform)
                        .with(GlobalTransform::default())
                        .with(Parent { entity: parent })
                        .build();
                }
                TableOrientation::HorizontalTop | TableOrientation::HorizontalBottom => {
                    let mut transform = Transform::default();
                    transform.translation = Vector3::new((x + 1.0) * 8.0, y * 8.0, 0.0);

                    world
                        .create_entity()
                        .with(SpriteRender {
                            sprite_sheet: map_preview_handle.clone(),
                            sprite_number: frame,
                            flip_horizontal: false,
                            flip_vertical: false,
                        })
                        .with(transform)
                        .with(GlobalTransform::default())
                        .with(Parent { entity: parent })
                        .build();
                }

                _ => {}
            },
        }
    });

    parent
}

pub fn create_map_from_file(
    world: &mut World,
    (left_parent, right_parent): (Entity, Entity),
    map_def: &MapDefinition,
    flavors: &[FlavorIndex],
    preparations: &[PreparationIndex],
    toppings: &[ToppingIndex],
) -> (Vec<Entity>, Vec<(f32, f32)>) {
    if flavors.len() > map_def.count_flavor_tables() {
        warn!("More flavors than the map has tables!");
    }
    if preparations.len() > map_def.count_preparation_tables() {
        warn!("More preparations than the map has tables!");
    }
    if toppings.len() > map_def.count_topping_tables() {
        warn!("More toppings than the map has tables!");
    }

    let map_handle = {
        let handles = world.read_resource::<Handles>();
        handles.map_handle.clone()
    };

    let tiles = vec![left_parent, right_parent]
        //let tiles = vec![left_parent]
        .iter()
        .map(|parent| {
            let mut tiles: Vec<Entity> = (0..10)
                .map(|i| {
                    (0..11)
                        .map(|j| {
                            let mut transform = Transform::default();
                            transform.translation =
                                Vector3::new((i as f32 + 0.5) * BASE, (j as f32) * BASE, -1019.0);
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
                                .with(Parent { entity: *parent })
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
                        *parent,
                        flavors,
                        preparations,
                        toppings,
                        t,
                        o,
                        *x * BASE,
                        *y * BASE,
                    )
                })
                .collect();

            let mut tops: Vec<Entity> = all.iter().map(|(top, _side)| *top).collect();

            let mut sides: Vec<Entity> = all.iter().map(|(_top, side)| *side).collect();

            tiles.append(&mut tops);
            tiles.append(&mut sides);
            tiles
        })
        .fold(Vec::new(), |mut acc, v| {
            acc.extend(v);
            acc
        });

    (
        tiles,
        map_def
            .spawns
            .iter()
            .map(|(x, y)| (*x * BASE, *y * BASE))
            .collect(),
    )
}

fn create_table(
    world: &mut World,
    parent: Entity,
    flavors: &[FlavorIndex],
    preparations: &[PreparationIndex],
    toppings: &[ToppingIndex],
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
        //t.set_rotation(Deg(0.0), Deg(0.0), o.make_rot());
        t
    };

    let side = |x, y, half_w, _half_h| {
        let mut t = Transform::default();
        t.translation = Vector3::new(x + half_w, y - (BASE / 4.0), 0.0);
        //t.scale = Vector3::new(1.0, 1.0, 1.0);
        t
    };

    if let TableType::Empty = t {
        let (w, h) = o.make_dim(BASE, BASE);
        let (half_w, half_h) = (w / 2.0, h / 2.0);

        let hitbox = hitbox(half_w, half_h);
        let top = top(x, y, half_w, half_h);
        let mut side = side(x, y, half_w, half_h);
        side.scale = Vector3::new(1.0, 1.0, 1.0);

        create_entities(
            world,
            parent,
            hitbox,
            side,
            top,
            (String::from("empty"), o.make_orientation_string()),
            Some(Table::new_empty_table()),
            false,
        )
    } else {
        let (w, h) = o.make_dim(BASE * 2.0, BASE);
        let (half_w, half_h) = (w / 2.0, h / 2.0);

        let hitbox = hitbox(half_w, half_h);
        let top = top(x, y, half_w, half_h);
        let side = side(x, y, half_w, half_h);

        match t {
            TableType::Flavor(f) => {
                let (key, table) = if let Some(my_flavor) = flavors.get(f.0) {
                    (
                        world
                            .read_resource::<Definitions>()
                            .flavors()
                            .find(|x| x.index == *my_flavor)
                            .unwrap()
                            .key
                            .clone(),
                        Some(Table::new_flavor_table(my_flavor.clone())),
                    )
                } else {
                    ("vanilla".to_owned(), None)
                };
                create_entities(
                    world,
                    parent,
                    hitbox,
                    side,
                    top,
                    (format!("flavor_{}", key), o.make_orientation_string()),
                    table,
                    true,
                )
            }
            TableType::Preparation(p) => {
                let (key, table) = if let Some(my_preparation) = preparations.get(p.0) {
                    (
                        world
                            .read_resource::<Definitions>()
                            .preparations()
                            .find(|x| x.index == *my_preparation)
                            .unwrap()
                            .key
                            .clone(),
                        Some(Table::new_preparation_table(my_preparation.clone())),
                    )
                } else {
                    ("cake_cone".to_owned(), None)
                };
                create_entities(
                    world,
                    parent,
                    hitbox,
                    side,
                    top,
                    (format!("preparation_{}", key), o.make_orientation_string()),
                    table,
                    true,
                )
            }
            TableType::Topping(t) => {
                let (key, table) = if let Some(my_topping) = toppings.get(t.0) {
                    (
                        world
                            .read_resource::<Definitions>()
                            .toppings()
                            .find(|x| x.index == *my_topping)
                            .unwrap()
                            .key
                            .clone(),
                        Some(Table::new_topping_table(my_topping.clone())),
                    )
                } else {
                    ("sprinkles".to_owned(), None)
                };
                create_entities(
                    world,
                    parent,
                    hitbox,
                    side,
                    top,
                    (format!("topping_{}", key), o.make_orientation_string()),
                    table,
                    true,
                )
            }
            TableType::Delivery => create_entities(
                world,
                parent,
                hitbox,
                side,
                top,
                (String::from("delivery"), o.make_orientation_string()),
                Some(Table::new_delivery_table()),
                true,
            ),

            _ => panic!("Impossible!"),
        }
    }
}

fn create_entities(
    world: &mut World,
    parent: Entity,
    hitbox: Hitbox,
    side: Transform,
    top: Transform,
    (key, orientation): (String, String),
    table: Option<Table>,
    animate: bool,
) -> (Entity, Entity) {
    let empty_handle = {
        let handles = world.read_resource::<Handles>();
        handles.empty_handle.clone()
    };

    let map_handle = {
        let handles = world.read_resource::<Handles>();
        handles.map_handle.clone()
    };

    let top = world
        .create_entity()
        .with(SpriteRender {
            sprite_sheet: empty_handle,
            sprite_number: 0,
            flip_horizontal: false,
            flip_vertical: false,
        })
        .with(Layered)
        .with(top)
        .with(Parent { entity: parent })
        .with(GlobalTransform::default());
    let top = if let Some(c) = table {
        top.with(c).build()
    } else {
        top.build()
    };

    let side = world
        .create_entity()
        .with(SpriteRender {
            sprite_sheet: map_handle,
            sprite_number: 7,
            flip_horizontal: false,
            flip_vertical: false,
        })
        .with(Solid)
        .with(hitbox)
        .with(Interact {
            highlighted_by: None,
            top,
        })
        .with(Layered)
        .with(side)
        .with(Parent { entity: parent })
        .with(GlobalTransform::default());
    let side = if animate {
        side.with(AnimatedTable(key, orientation)).build()
    } else {
        side.build()
    };

    (side, top)
}
