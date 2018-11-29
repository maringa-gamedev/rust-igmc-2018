use super::*;
use amethyst::{
    assets::Loader,
    core::{
        cgmath::*,
        transform::{GlobalTransform, Parent, ParentHierarchy, Transform},
    },
    ecs::prelude::*,
    input::{is_close_requested, is_key_down},
    prelude::*,
    renderer::{
        Camera, Hidden, Projection, ScreenDimensions, SpriteRender, Transparent, VirtualKeyCode,
    },
    ui::{Anchor, FontAsset, FontHandle, TtfFormat, UiFinder, UiText, UiTransform},
    utils::application_root_dir,
};
use either::*;
use log::*;
use nalgebra::Vector2 as NAVector2;
use ncollide2d::shape::*;
use nk_data::*;
use nk_ecs::*;
use nk_loader::*;
use nk_util::*;
use std::iter;

pub struct FreePlay {
    flavors: Vec<(FlavorIndex, bool)>,
    preparations: Vec<(PreparationIndex, bool)>,
    toppings: Vec<(ToppingIndex, bool)>,

    selected_flavors: Vec<FlavorIndex>,
    selected_preparation: usize,
    selected_topping: usize,

    maps: Vec<(MapDefinition, Option<Entity>, bool)>,
    selected_map: usize,

    current_selection: usize,
    row: usize,
    page: bool,

    map_screen: Option<Entity>,
    loadout_screen: Option<Entity>,
    entities: Vec<Entity>,
}

impl Default for FreePlay {
    fn default() -> Self {
        FreePlay {
            flavors: vec![
                (FlavorIndex(0), true),
                (FlavorIndex(1), true),
                (FlavorIndex(2), true),
                (FlavorIndex(4), true),
                (FlavorIndex(5), true),
                (FlavorIndex(6), true),
                (FlavorIndex(7), true),
                (FlavorIndex(8), true),
                (FlavorIndex(9), true),
                (FlavorIndex(10), false),
                (FlavorIndex(11), false),
                (FlavorIndex(12), false),
                (FlavorIndex(13), false),
                (FlavorIndex(15), false),
                (FlavorIndex(16), false),
                (FlavorIndex(17), false),
                (FlavorIndex(18), false),
                (FlavorIndex(19), false),
                (FlavorIndex(20), false),
                (FlavorIndex(21), false),
                (FlavorIndex(22), false),
                (FlavorIndex(24), false),
                (FlavorIndex(25), false),
                (FlavorIndex(26), false),
                (FlavorIndex(27), false),
                (FlavorIndex(28), false),
                (FlavorIndex(29), false),
                (FlavorIndex(30), false),
                (FlavorIndex(31), false),
            ],
            preparations: vec![
                (PreparationIndex(0), true),
                (PreparationIndex(1), false),
                (PreparationIndex(2), false),
                (PreparationIndex(3), false),
                (PreparationIndex(4), false),
                (PreparationIndex(5), false),
                (PreparationIndex(6), false),
                (PreparationIndex(7), false),
                (PreparationIndex(8), false),
                (PreparationIndex(9), false),
                (PreparationIndex(10), false),
                (PreparationIndex(11), false),
            ],
            toppings: vec![
                (ToppingIndex(0), true),
                (ToppingIndex(1), false),
                (ToppingIndex(2), false),
                (ToppingIndex(3), false),
                (ToppingIndex(4), false),
                (ToppingIndex(5), false),
                (ToppingIndex(6), false),
                (ToppingIndex(7), false),
                (ToppingIndex(8), false),
                (ToppingIndex(9), false),
                (ToppingIndex(10), false),
                (ToppingIndex(11), false),
                (ToppingIndex(12), false),
                (ToppingIndex(13), false),
            ],
            selected_flavors: Vec::with_capacity(4),
            selected_preparation: 0,
            selected_topping: 0,
            row: 0,
            page: false,

            maps: load_freeplay_defs()
                .into_iter()
                .map(|m| (m, None, true))
                .collect(),
            selected_map: 0,

            current_selection: 0,

            map_screen: None,
            loadout_screen: None,
            entities: Vec::with_capacity(128),
        }
    }
}

impl<'a, 'b> SimpleState<'a, 'b> for FreePlay {
    fn on_start(&mut self, data: StateData<GameData>) {
        let StateData { mut world, .. } = data;

        world.register::<UiFlavor>();
        world.register::<UiPreparation>();
        world.register::<UiTopping>();
        world.register::<MapSelection>();

        self.maps = self
            .maps
            .iter()
            .map(|(m, e, b)| {
                if let Some(e) = e {
                    (m.clone(), Some(*e), *b)
                } else {
                    let e = create_map_preview(&mut world, m);
                    (m.clone(), Some(e), *b)
                }
            })
            .collect();

        let (map_ui, loadout_ui) = load_freeplay_ui();

        self.map_screen = Some(generate_ui(&mut world, &mut self.entities, &map_ui));
        self.loadout_screen = Some(generate_ui(&mut world, &mut self.entities, &loadout_ui));
    }

    fn handle_event(
        &mut self,
        _data: StateData<GameData>,
        event: StateEvent,
    ) -> SimpleTrans<'a, 'b> {
        if let StateEvent::Window(event) = &event {
            if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Q) {
                return Trans::Quit;
            }
            if is_key_down(&event, VirtualKeyCode::Return) {
                if self.page {
                    if self.selected_flavors.len() == 4 {
                        return Trans::Switch(Box::new(
                            Game::with_map(self.maps[self.selected_map].0.clone())
                                .with_flavors(
                                    self.flavors
                                        .iter()
                                        .filter_map(|x| {
                                            if self.selected_flavors.iter().any(|y| *y == x.0) {
                                                Some(x.0.clone())
                                            } else {
                                                None
                                            }
                                        })
                                        .fold(Vec::with_capacity(8), |mut acc, v| {
                                            acc.push(v.clone());
                                            acc.push(v);
                                            acc
                                        }),
                                )
                                .with_preparations(
                                    iter::repeat(
                                        self.preparations[self.selected_preparation].0.clone(),
                                    )
                                    .take(4)
                                    .collect(),
                                )
                                .with_toppings(
                                    iter::repeat(self.toppings[self.selected_topping].0.clone())
                                        .take(4)
                                        .collect(),
                                ),
                        ));
                    }
                } else {
                    self.page = true;
                }
            }
            if is_key_down(&event, VirtualKeyCode::Back) {
                if self.page {
                    self.page = false;
                }
            }
            if is_key_down(&event, VirtualKeyCode::Right) {
                if self.page {
                    self.current_selection = match self.row {
                        0 => (self.current_selection + 1) % 9,
                        1 => (self.current_selection + 1) % 5,
                        2 => (self.current_selection + 1) % 8,
                        3 => (self.current_selection + 1) % 7,
                        4 => (self.current_selection + 1) % 12,
                        5 => (self.current_selection + 1) % 14,
                        _ => std::unreachable!(),
                    };
                } else {
                    let last = self.maps.len() - 1;
                    self.selected_map = match self.selected_map {
                        _x if _x == last => last,
                        x => x + 1,
                    };
                }
            }
            if is_key_down(&event, VirtualKeyCode::Left) {
                if self.page {
                    self.current_selection = if self.current_selection > 0 {
                        match self.row {
                            0..6 => self.current_selection - 1,
                            _ => std::unreachable!(),
                        }
                    } else {
                        match self.row {
                            0 => 8,
                            1 => 4,
                            2 => 7,
                            3 => 6,
                            4 => 11,
                            5 => 13,
                            _ => std::unreachable!(),
                        }
                    };
                } else {
                    let last = self.maps.len() - 1;
                    self.selected_map = match self.selected_map {
                        _x if _x == 0 => 0,
                        x => x - 1,
                    };
                }
            }
            if is_key_down(&event, VirtualKeyCode::Down) {
                if self.page {
                    self.row = match self.row {
                        0..5 => self.row + 1,
                        5 => 0,
                        _ => std::unreachable!(),
                    };
                    self.current_selection = 0;
                }
            }
            if is_key_down(&event, VirtualKeyCode::Up) {
                if self.page {
                    self.row = match self.row {
                        1..6 => self.row - 1,
                        0 => 5,
                        _ => std::unreachable!(),
                    };
                    self.current_selection = 0;
                }
            }
            if is_key_down(&event, VirtualKeyCode::Space) {
                if self.page {
                    match self.row {
                        0..4 => {
                            let acc = match self.row {
                                0 => 0,
                                1 => 8,
                                2 => 8 + 4,
                                3 => 8 + 4 + 7,
                                _ => std::unreachable!(),
                            } + self.current_selection;
                            let current = &self.flavors[acc];
                            if self.selected_flavors.iter().any(|x| *x == current.0) {
                                self.selected_flavors.remove_item(&current.0);
                            } else if current.1 && self.selected_flavors.len() < 4 {
                                self.selected_flavors.push(current.0.clone());
                            }
                        }
                        4 => {
                            let current = &self.preparations[self.current_selection];
                            if current.1 {
                                self.selected_preparation = self.current_selection;
                            }
                        }
                        5 => {
                            let current = &self.toppings[self.current_selection];
                            if current.1 {
                                self.selected_topping = self.current_selection;
                            }
                        }
                        _ => std::unreachable!(),
                    };
                }
            }
        }
        Trans::None
    }

    fn update(
        &mut self,
        StateData {
            ref mut world,
            data,
        }: &mut StateData<GameData>,
    ) -> SimpleTrans<'a, 'b> {
        //if let Some(camera) = self.camera.take() {
        //super::update_viewport(camera, world);
        //}

        let (
            ui_box_orange,
            ui_box_green,
            ui_box_green_hl,
            ui_box_gray,
            ui_box_gray_hl,
            ui_box_white,
            white_arrow_left,
            white_arrow_right,
            gray_arrow_left,
            gray_arrow_right,
        ) = {
            let anims = world.read_resource::<Animations>();
            let anims = &anims.animations;
            (
                anims["ui_box_orange"].get_frame(),
                anims["ui_box_green"].get_frame(),
                anims["ui_box_green_hl"].get_frame(),
                anims["ui_box_gray"].get_frame(),
                anims["ui_box_gray_hl"].get_frame(),
                anims["ui_box_white"].get_frame(),
                anims["white_arrow_left"].get_frame(),
                anims["white_arrow_right"].get_frame(),
                anims["gray_arrow_left"].get_frame(),
                anims["gray_arrow_right"].get_frame(),
            )
        };

        let parents = world.read_resource::<ParentHierarchy>();
        let ui_flavors = world.read_storage::<UiFlavor>();
        let ui_preparations = world.read_storage::<UiPreparation>();
        let ui_toppings = world.read_storage::<UiTopping>();
        let map_selections = world.read_storage::<MapSelection>();
        let mut hiddens = world.write_storage::<Hidden>();
        let mut transforms = world.write_storage::<Transform>();
        let mut sprites = world.write_storage::<SpriteRender>();

        for map_preview in &self.maps {
            if let Some(entity) = map_preview.1 {
                for child in parents.children(entity) {
                    if let None = hiddens.get(*child) {
                        hiddens.insert(*child, Hidden).unwrap();
                    }
                }
            }
        }

        if let Some(parent) = self.map_screen {
            for child in parents.children(parent) {
                if let None = hiddens.get(*child) {
                    if self.page {
                        hiddens.insert(*child, Hidden).unwrap();
                    }
                } else {
                    if !self.page {
                        hiddens.remove(*child);
                    }
                }
                let (x, y) = {
                    let parent_transform = transforms.get(*child).unwrap();
                    (
                        parent_transform.translation.x,
                        parent_transform.translation.y,
                    )
                };
                let sprite = sprites.get_mut(*child).unwrap();
                match map_selections.get(*child) {
                    Some(MapSelection(MapFunction::ArrowPrevious)) => {
                        sprite.sprite_number = if self.selected_map == 0 {
                            gray_arrow_left
                        } else {
                            white_arrow_left
                        };
                    }
                    Some(MapSelection(MapFunction::ArrowNext)) => {
                        sprite.sprite_number = if self.selected_map == self.maps.len() - 1 {
                            gray_arrow_right
                        } else {
                            white_arrow_right
                        };
                    }
                    Some(MapSelection(MapFunction::PreviousDisplay)) => {
                        if self.selected_map > 0 {
                            let option_parent = self.maps[self.selected_map - 1].1;
                            if let Some(entity) = option_parent {
                                let transform = transforms.get_mut(entity).unwrap();
                                transform.translation.x = x;
                                transform.translation.y = y;
                                transform.translation.z = 4.0;
                                transform.scale.x = 0.5;
                                transform.scale.y = 0.5;
                                if !self.page {
                                    for child in parents.children(entity) {
                                        if let Some(_) = hiddens.get(*child) {
                                            hiddens.remove(*child);
                                        }
                                    }
                                } else {
                                    for child in parents.children(entity) {
                                        if let None = hiddens.get(*child) {
                                            hiddens.insert(*child, Hidden).unwrap();
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Some(MapSelection(MapFunction::CurrentDisplay)) => {
                        let option_parent = self.maps[self.selected_map].1;
                        if let Some(entity) = option_parent {
                            let transform = transforms.get_mut(entity).unwrap();
                            transform.translation.x = x;
                            transform.translation.y = y;
                            transform.translation.z = 4.0;
                            transform.scale.x = 1.0;
                            transform.scale.y = 1.0;
                            if !self.page {
                                for child in parents.children(entity) {
                                    if let Some(_) = hiddens.get(*child) {
                                        hiddens.remove(*child);
                                    }
                                }
                            } else {
                                for child in parents.children(entity) {
                                    if let None = hiddens.get(*child) {
                                        hiddens.insert(*child, Hidden).unwrap();
                                    }
                                }
                            }
                        }
                    }
                    Some(MapSelection(MapFunction::NextDisplay)) => {
                        if self.selected_map != self.maps.len() - 1 {
                            let option_parent = self.maps[self.selected_map + 1].1;
                            if let Some(entity) = option_parent {
                                let transform = transforms.get_mut(entity).unwrap();
                                transform.translation.x = x;
                                transform.translation.y = y;
                                transform.translation.z = 4.0;
                                transform.scale.x = 0.5;
                                transform.scale.y = 0.5;
                                if !self.page {
                                    for child in parents.children(entity) {
                                        if let Some(_) = hiddens.get(*child) {
                                            hiddens.remove(*child);
                                        }
                                    }
                                } else {
                                    for child in parents.children(entity) {
                                        if let None = hiddens.get(*child) {
                                            hiddens.insert(*child, Hidden).unwrap();
                                        }
                                    }
                                }
                            }
                        }
                    }
                    None => {}
                }
            }
        }

        if let Some(parent) = self.loadout_screen {
            for child in parents.children(parent) {
                if let None = hiddens.get(*child) {
                    if !self.page {
                        hiddens.insert(*child, Hidden).unwrap();
                    }
                } else {
                    if self.page {
                        hiddens.remove(*child);
                    }
                }
                if let Some(UiFlavor(r, i, f)) = ui_flavors.get(*child) {
                    let sprite = sprites.get_mut(*child).unwrap();
                    if self.flavors.iter().any(|x| x.0 == *f && x.1 == false) {
                        if *r == self.row && *i == self.current_selection {
                            sprite.sprite_number = ui_box_gray_hl;
                        } else {
                            sprite.sprite_number = ui_box_gray;
                        }
                    } else if self.selected_flavors.iter().any(|x| x == f) {
                        if *r == self.row && *i == self.current_selection {
                            sprite.sprite_number = ui_box_green_hl;
                        } else {
                            sprite.sprite_number = ui_box_green;
                        }
                    } else if *r == self.row && *i == self.current_selection {
                        sprite.sprite_number = ui_box_orange;
                    } else {
                        sprite.sprite_number = ui_box_white;
                    }
                } else if let Some(UiPreparation(r, i, p)) = ui_preparations.get(*child) {
                    let sprite = sprites.get_mut(*child).unwrap();
                    if self.preparations.iter().any(|x| x.0 == *p && x.1 == false) {
                        if *r == self.row && *i == self.current_selection {
                            sprite.sprite_number = ui_box_gray_hl;
                        } else {
                            sprite.sprite_number = ui_box_gray;
                        }
                    } else if self.preparations[self.selected_preparation].0 == *p {
                        if *r == self.row && *i == self.current_selection {
                            sprite.sprite_number = ui_box_green_hl;
                        } else {
                            sprite.sprite_number = ui_box_green;
                        }
                    } else if *r == self.row && *i == self.current_selection {
                        sprite.sprite_number = ui_box_orange;
                    } else {
                        sprite.sprite_number = ui_box_white;
                    }
                } else if let Some(UiTopping(r, i, t)) = ui_toppings.get(*child) {
                    let sprite = sprites.get_mut(*child).unwrap();
                    if self.toppings.iter().any(|x| x.0 == *t && x.1 == false) {
                        if *r == self.row && *i == self.current_selection {
                            sprite.sprite_number = ui_box_gray_hl;
                        } else {
                            sprite.sprite_number = ui_box_gray;
                        }
                    } else if self.toppings[self.selected_topping].0 == *t {
                        if *r == self.row && *i == self.current_selection {
                            sprite.sprite_number = ui_box_green_hl;
                        } else {
                            sprite.sprite_number = ui_box_green;
                        }
                    } else if *r == self.row && *i == self.current_selection {
                        sprite.sprite_number = ui_box_orange;
                    } else {
                        sprite.sprite_number = ui_box_white;
                    }
                }
            }
        }

        Trans::None
    }

    fn on_stop(&mut self, data: StateData<GameData>) {
        let StateData { world, .. } = data;
        world
            .delete_entities(self.entities.as_slice())
            .expect("Failed to clean world of FreePlay's entities!");
    }
}
