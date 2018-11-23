use amethyst::ecs::prelude::*;
use crate::component::*;
use either::*;
use nk_data::*;

#[derive(Debug)]
pub enum Style {
    HalfLeft,
    HalfRight,
    Full,
}

pub type EitherPrepTop = Either<PreparationInteraction, ToppingInteraction>;
pub type EitherInteractions = Either<FlavorInteraction, EitherPrepTop>;

pub struct Player {
    pub gamepad_index: usize,
    pub gamepad_style: Style,
    pub layer: f32,
    pub inventory: Option<Either<FlavorIndex, Order>>,
    pub invert_x_axis: bool,
    pub team_index: usize,
    pub interaction: Option<Entity>,
    pub palette_key: String,
}

impl Player {
    pub fn new(
        gamepad_index: usize,
        team_index: usize,
        layer: f32,
        invert_x_axis: bool,
        palette_key: String,
    ) -> Self {
        Player {
            gamepad_index,
            gamepad_style: Style::Full,
            layer,
            inventory: None,
            invert_x_axis,
            team_index,
            interaction: None,
            palette_key,
        }
    }

    pub fn new_with_style(
        gamepad_index: usize,
        gamepad_style: Style,
        team_index: usize,
        layer: f32,
        invert_x_axis: bool,
        palette_key: String,
    ) -> Self {
        Player {
            gamepad_index,
            gamepad_style,
            layer,
            inventory: None,
            invert_x_axis,
            team_index,
            interaction: None,
            palette_key,
        }
    }
}

impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}
