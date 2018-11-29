use amethyst::{
    core::{
        cgmath::*,
        transform::{GlobalTransform, Parent, Transform},
    },
    ecs::prelude::*,
    renderer::{SpriteRender, SpriteSheetHandle, Transparent},
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

pub fn load_freeplay_ui() -> (UiDefinition, UiDefinition) {
    let app_root = application_root_dir();
    let path = format!("{}/assets/texture/ui/map_selection.ron", app_root);
    let f = File::open(&path).expect("Failed opening file");
    let map_selection = match from_reader(f) {
        Ok(x) => x,
        Err(e) => {
            error!("Error parsing freeplay ui file: {}", e);
            panic!("Invalid map selection ui file <{}>!", path);
        }
    };

    let path = format!("{}/assets/texture/ui/loadout_selection.ron", app_root);
    let f = File::open(&path).expect("Failed opening file");
    let loadout_selection = match from_reader(f) {
        Ok(x) => x,
        Err(e) => {
            error!("Error parsing freeplay ui file: {}", e);
            panic!("Invalid loadout selection ui file <{}>!", path);
        }
    };

    (map_selection, loadout_selection)
}

pub fn generate_ui(world: &mut World, entities: &mut Vec<Entity>, ui: &UiDefinition) -> Entity {
    let (
        empty_handle,
        empty_frame,
        ui_handle,
        gray_arrow_up,
        gray_arrow_down,
        gray_arrow_right,
        gray_arrow_left,
        white_arrow_up,
        white_arrow_down,
        white_arrow_right,
        white_arrow_left,
        button_confirm_normal,
        button_confirm_down,
        button_back_normal,
        button_back_down,
        ui_cross_icon,
        ui_circle_icon,
        ui_unknown,
        ui_box_orange,
        ui_box_green,
        ui_box_gray,
        ui_box_white,
        ui_full_corner_top_left,
        ui_full_corner_top_right,
        ui_full_corner_bottom_left,
        ui_full_corner_bottom_right,
        ui_full_side_top,
        ui_full_side_left,
        ui_full_side_bottom,
        ui_full_side_right,
        ui_half_side_top,
        ui_half_side_left,
        ui_half_side_bottom,
        ui_half_side_right,
        ui_full_center,
        ui_half_corner_top_left,
        ui_half_corner_top_right,
        ui_half_corner_bottom_left,
        ui_half_corner_bottom_right,
        ui_cut_corner_top_left,
        ui_cut_corner_top_right,
        ui_cut_corner_bottom_left,
        ui_cut_corner_bottom_right,
        ui_connect_top_left,
        ui_connect_top_right,
        ui_connect_bottom_left,
        ui_connect_bottom_right,
    ) = {
        let anims = world.read_resource::<Animations>();
        let anims = &anims.animations;
        (
            anims["empty_item"].obtain_handle(),
            anims["empty_item"].get_frame(),
            anims["map_preview_floor"].obtain_handle(),
            anims["gray_arrow_up"].get_frame(),
            anims["gray_arrow_down"].get_frame(),
            anims["gray_arrow_right"].get_frame(),
            anims["gray_arrow_left"].get_frame(),
            anims["white_arrow_up"].get_frame(),
            anims["white_arrow_down"].get_frame(),
            anims["white_arrow_right"].get_frame(),
            anims["white_arrow_left"].get_frame(),
            anims["button_confirm_normal"].get_frame(),
            anims["button_confirm_down"].get_frame(),
            anims["button_back_normal"].get_frame(),
            anims["button_back_down"].get_frame(),
            anims["ui_cross_icon"].get_frame(),
            anims["ui_circle_icon"].get_frame(),
            anims["ui_unknown"].get_frame(),
            anims["ui_box_orange"].get_frame(),
            anims["ui_box_green"].get_frame(),
            anims["ui_box_gray"].get_frame(),
            anims["ui_box_white"].get_frame(),
            anims["ui_full_corner_top_left"].get_frame(),
            anims["ui_full_corner_top_right"].get_frame(),
            anims["ui_full_corner_bottom_left"].get_frame(),
            anims["ui_full_corner_bottom_right"].get_frame(),
            anims["ui_full_side_top"].get_frame(),
            anims["ui_full_side_left"].get_frame(),
            anims["ui_full_side_bottom"].get_frame(),
            anims["ui_full_side_right"].get_frame(),
            anims["ui_half_side_top"].get_frame(),
            anims["ui_half_side_left"].get_frame(),
            anims["ui_half_side_bottom"].get_frame(),
            anims["ui_half_side_right"].get_frame(),
            anims["ui_full_center"].get_frame(),
            anims["ui_half_corner_top_left"].get_frame(),
            anims["ui_half_corner_top_right"].get_frame(),
            anims["ui_half_corner_bottom_left"].get_frame(),
            anims["ui_half_corner_bottom_right"].get_frame(),
            anims["ui_cut_corner_top_left"].get_frame(),
            anims["ui_cut_corner_top_right"].get_frame(),
            anims["ui_cut_corner_bottom_left"].get_frame(),
            anims["ui_cut_corner_bottom_right"].get_frame(),
            anims["ui_connect_top_left"].get_frame(),
            anims["ui_connect_top_right"].get_frame(),
            anims["ui_connect_bottom_left"].get_frame(),
            anims["ui_connect_bottom_right"].get_frame(),
        )
    };

    let mut transform = Transform::default();
    transform.translation = Vector3::new(0.0, 0.0, 1.0);

    let parent = world
        .create_entity()
        .with(transform)
        .with(GlobalTransform::default())
        .build();

    ui.sprites.iter().for_each(|UiElement(x, y, elem)| {
        let mut transform = Transform::default();
        transform.translation = Vector3::new(x * 16.0, y * 16.0, 0.0);

        match elem {
            UiSprite::FullCorner(corner) => {
                let (sprite_sheet, sprite_number) = match corner {
                    UiCorner::TopLeft => (ui_handle.clone(), ui_full_corner_top_left),
                    UiCorner::TopRight => (ui_handle.clone(), ui_full_corner_top_right),
                    UiCorner::BottomLeft => (ui_handle.clone(), ui_full_corner_bottom_left),
                    UiCorner::BottomRight => (ui_handle.clone(), ui_full_corner_bottom_right),
                };
                entities.push(
                    world
                        .create_entity()
                        .with(SpriteRender {
                            sprite_sheet,
                            sprite_number,
                            flip_horizontal: false,
                            flip_vertical: false,
                        })
                        .with(Transparent)
                        .with(transform)
                        .with(GlobalTransform::default())
                        .with(Parent { entity: parent })
                        .build(),
                );
            }

            UiSprite::HalfCorner(corner) => {
                let (sprite_sheet, sprite_number) = match corner {
                    UiCorner::TopLeft => (ui_handle.clone(), ui_half_corner_top_left),
                    UiCorner::TopRight => (ui_handle.clone(), ui_half_corner_top_right),
                    UiCorner::BottomLeft => (ui_handle.clone(), ui_half_corner_bottom_left),
                    UiCorner::BottomRight => (ui_handle.clone(), ui_half_corner_bottom_right),
                };
                entities.push(
                    world
                        .create_entity()
                        .with(SpriteRender {
                            sprite_sheet,
                            sprite_number,
                            flip_horizontal: false,
                            flip_vertical: false,
                        })
                        .with(Transparent)
                        .with(transform)
                        .with(GlobalTransform::default())
                        .with(Parent { entity: parent })
                        .build(),
                );
            }

            UiSprite::CutCorner(corner) => {
                let (sprite_sheet, sprite_number) = match corner {
                    UiCorner::TopLeft => (ui_handle.clone(), ui_cut_corner_top_left),
                    UiCorner::TopRight => (ui_handle.clone(), ui_cut_corner_top_right),
                    UiCorner::BottomLeft => (ui_handle.clone(), ui_cut_corner_bottom_left),
                    UiCorner::BottomRight => (ui_handle.clone(), ui_cut_corner_bottom_right),
                };
                entities.push(
                    world
                        .create_entity()
                        .with(SpriteRender {
                            sprite_sheet,
                            sprite_number,
                            flip_horizontal: false,
                            flip_vertical: false,
                        })
                        .with(Transparent)
                        .with(transform)
                        .with(GlobalTransform::default())
                        .with(Parent { entity: parent })
                        .build(),
                );
            }

            UiSprite::ConnectCorner(corner) => {
                let (sprite_sheet, sprite_number) = match corner {
                    UiCorner::TopLeft => (ui_handle.clone(), ui_connect_top_left),
                    UiCorner::TopRight => (ui_handle.clone(), ui_connect_top_right),
                    UiCorner::BottomLeft => (ui_handle.clone(), ui_connect_bottom_left),
                    UiCorner::BottomRight => (ui_handle.clone(), ui_connect_bottom_right),
                };
                entities.push(
                    world
                        .create_entity()
                        .with(SpriteRender {
                            sprite_sheet,
                            sprite_number,
                            flip_horizontal: false,
                            flip_vertical: false,
                        })
                        .with(Transparent)
                        .with(transform)
                        .with(GlobalTransform::default())
                        .with(Parent { entity: parent })
                        .build(),
                );
            }

            UiSprite::FullSide(side) => {
                let (sprite_sheet, sprite_number) = match side {
                    UiSide::Top => (ui_handle.clone(), ui_full_side_top),
                    UiSide::Left => (ui_handle.clone(), ui_full_side_left),
                    UiSide::Bottom => (ui_handle.clone(), ui_full_side_bottom),
                    UiSide::Right => (ui_handle.clone(), ui_full_side_right),
                };
                entities.push(
                    world
                        .create_entity()
                        .with(SpriteRender {
                            sprite_sheet,
                            sprite_number,
                            flip_horizontal: false,
                            flip_vertical: false,
                        })
                        .with(Transparent)
                        .with(transform)
                        .with(GlobalTransform::default())
                        .with(Parent { entity: parent })
                        .build(),
                );
            }

            UiSprite::HalfSide(side) => {
                let (sprite_sheet, sprite_number) = match side {
                    UiSide::Top => (ui_handle.clone(), ui_half_side_top),
                    UiSide::Left => (ui_handle.clone(), ui_half_side_left),
                    UiSide::Bottom => (ui_handle.clone(), ui_half_side_bottom),
                    UiSide::Right => (ui_handle.clone(), ui_half_side_right),
                };
                entities.push(
                    world
                        .create_entity()
                        .with(SpriteRender {
                            sprite_sheet,
                            sprite_number,
                            flip_horizontal: false,
                            flip_vertical: false,
                        })
                        .with(Transparent)
                        .with(transform)
                        .with(GlobalTransform::default())
                        .with(Parent { entity: parent })
                        .build(),
                );
            }

            UiSprite::Center => {
                let (sprite_sheet, sprite_number) = (ui_handle.clone(), ui_full_center);
                entities.push(
                    world
                        .create_entity()
                        .with(SpriteRender {
                            sprite_sheet,
                            sprite_number,
                            flip_horizontal: false,
                            flip_vertical: false,
                        })
                        .with(Transparent)
                        .with(transform)
                        .with(GlobalTransform::default())
                        .with(Parent { entity: parent })
                        .build(),
                );
            }

            UiSprite::ToggleFlavor(r, i, f) => {
                let (sprite_sheet, sprite_number) = {
                    let defs = world.read_resource::<Definitions>();
                    let anims = world.read_resource::<Animations>();
                    let anims = &anims.animations;
                    let key = &defs.flavors().find(|x| x.index == *f).unwrap().key;
                    let key = format!("{}_ball", key);
                    info!("{}", key);
                    (anims[&key].obtain_handle(), anims[&key].get_frame())
                };
                entities.push(
                    world
                        .create_entity()
                        .with(SpriteRender {
                            sprite_sheet: ui_handle.clone(),
                            sprite_number: ui_box_white,
                            flip_horizontal: false,
                            flip_vertical: false,
                        })
                        .with(UiFlavor(*r, *i, f.clone()))
                        .with(Transparent)
                        .with(transform)
                        .with(GlobalTransform::default())
                        .with(Parent { entity: parent })
                        .build(),
                );

                let mut transform = Transform::default();
                transform.translation = Vector3::new(8.0 + x * 16.0, 8.0 + y * 16.0, 0.0);

                entities.push(
                    world
                        .create_entity()
                        .with(SpriteRender {
                            sprite_sheet,
                            sprite_number,
                            flip_horizontal: false,
                            flip_vertical: false,
                        })
                        .with(Transparent)
                        .with(transform)
                        .with(GlobalTransform::default())
                        .with(Parent { entity: parent })
                        .build(),
                );
            }

            UiSprite::TogglePreparation(r, i, p) => {
                entities.push(
                    world
                        .create_entity()
                        .with(SpriteRender {
                            sprite_sheet: ui_handle.clone(),
                            sprite_number: ui_box_white,
                            flip_horizontal: false,
                            flip_vertical: false,
                        })
                        .with(UiPreparation(*r, *i, p.clone()))
                        .with(Transparent)
                        .with(transform.clone())
                        .with(GlobalTransform::default())
                        .with(Parent { entity: parent })
                        .build(),
                );

                entities.push(
                    world
                        .create_entity()
                        .with(SpriteRender {
                            sprite_sheet: ui_handle.clone(),
                            sprite_number: ui_unknown,
                            flip_horizontal: false,
                            flip_vertical: false,
                        })
                        .with(Transparent)
                        .with(transform)
                        .with(GlobalTransform::default())
                        .with(Parent { entity: parent })
                        .build(),
                );
            }

            UiSprite::ToggleTopping(r, i, t) => {
                entities.push(
                    world
                        .create_entity()
                        .with(SpriteRender {
                            sprite_sheet: ui_handle.clone(),
                            sprite_number: ui_box_white,
                            flip_horizontal: false,
                            flip_vertical: false,
                        })
                        .with(UiTopping(*r, *i, t.clone()))
                        .with(Transparent)
                        .with(transform.clone())
                        .with(GlobalTransform::default())
                        .with(Parent { entity: parent })
                        .build(),
                );

                entities.push(
                    world
                        .create_entity()
                        .with(SpriteRender {
                            sprite_sheet: ui_handle.clone(),
                            sprite_number: ui_unknown,
                            flip_horizontal: false,
                            flip_vertical: false,
                        })
                        .with(Transparent)
                        .with(transform)
                        .with(GlobalTransform::default())
                        .with(Parent { entity: parent })
                        .build(),
                );
            }

            UiSprite::MapPreview(index) => {
                let (sprite_sheet, sprite_number) = (empty_handle.clone(), empty_frame);
                entities.push(
                    world
                        .create_entity()
                        .with(SpriteRender {
                            sprite_sheet,
                            sprite_number,
                            flip_horizontal: false,
                            flip_vertical: false,
                        })
                        .with(MapSelection(match index {
                            UiIndex::Previous => MapFunction::PreviousDisplay,
                            UiIndex::Current => MapFunction::CurrentDisplay,
                            UiIndex::Next => MapFunction::NextDisplay,
                        }))
                        .with(Transparent)
                        .with(transform)
                        .with(GlobalTransform::default())
                        .with(Parent { entity: parent })
                        .build(),
                );
            }

            UiSprite::Arrow(dir) => {
                let (sprite_sheet, sprite_number) = match dir {
                    UiDirection::Previous => (ui_handle.clone(), white_arrow_left),
                    UiDirection::Next => (ui_handle.clone(), white_arrow_right),
                };
                entities.push(
                    world
                        .create_entity()
                        .with(SpriteRender {
                            sprite_sheet,
                            sprite_number,
                            flip_horizontal: false,
                            flip_vertical: false,
                        })
                        .with(MapSelection(match dir {
                            UiDirection::Previous => MapFunction::ArrowPrevious,
                            UiDirection::Next => MapFunction::ArrowNext,
                        }))
                        .with(Transparent)
                        .with(transform)
                        .with(GlobalTransform::default())
                        .with(Parent { entity: parent })
                        .build(),
                );
            }

            UiSprite::Button(action) => {
                let (sprite_sheet, sprite_number) = match action {
                    UiAction::Back => (ui_handle.clone(), button_back_normal),
                    UiAction::Confirm => (ui_handle.clone(), button_confirm_normal),
                };
                entities.push(
                    world
                        .create_entity()
                        .with(SpriteRender {
                            sprite_sheet,
                            sprite_number,
                            flip_horizontal: false,
                            flip_vertical: false,
                        })
                        .with(Transparent)
                        .with(transform)
                        .with(GlobalTransform::default())
                        .with(Parent { entity: parent })
                        .build(),
                );
            }
        }
    });

    parent
}
