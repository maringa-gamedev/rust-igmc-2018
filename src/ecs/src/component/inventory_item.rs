use amethyst::ecs::prelude::*;

pub struct InventoryItem;

impl Component for InventoryItem {
    type Storage = DenseVecStorage<Self>;
}
