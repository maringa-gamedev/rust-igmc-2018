use amethyst::ecs::prelude::*;

pub struct InventoryItem(pub usize);

impl Component for InventoryItem {
    type Storage = DenseVecStorage<Self>;
}
