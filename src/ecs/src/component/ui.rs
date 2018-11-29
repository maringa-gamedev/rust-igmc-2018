use amethyst::ecs::prelude::*;
use nk_data::*;

pub struct UiFlavor(pub usize, pub usize, pub FlavorIndex);
pub struct UiPreparation(pub usize, pub usize, pub PreparationIndex);
pub struct UiTopping(pub usize, pub usize, pub ToppingIndex);

impl Component for UiFlavor {
    type Storage = DenseVecStorage<Self>;
}

impl Component for UiPreparation {
    type Storage = DenseVecStorage<Self>;
}

impl Component for UiTopping {
    type Storage = DenseVecStorage<Self>;
}

pub enum MapFunction {
    PreviousDisplay,
    CurrentDisplay,
    NextDisplay,
    ArrowPrevious,
    ArrowNext,
}

pub struct MapSelection(pub MapFunction);

impl Component for MapSelection {
    type Storage = DenseVecStorage<Self>;
}
