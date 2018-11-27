use amethyst::ecs::prelude::*;

pub struct ScoreDigit(pub usize, pub bool);

impl Component for ScoreDigit {
    type Storage = DenseVecStorage<Self>;
}

pub struct TimerDigit(pub usize);

impl Component for TimerDigit {
    type Storage = DenseVecStorage<Self>;
}

// Team index first, Order index second
pub struct OrderSlot(pub usize, pub usize);

impl Component for OrderSlot {
    type Storage = DenseVecStorage<Self>;
}

pub struct OrderIngredient(pub usize);

impl Component for OrderIngredient {
    type Storage = DenseVecStorage<Self>;
}
