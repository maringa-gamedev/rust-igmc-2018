use amethyst::ecs::prelude::*;

pub struct AnimatedFloor(pub String);

impl Component for AnimatedFloor {
    type Storage = DenseVecStorage<Self>;
}

pub struct AnimatedTable(pub String, pub String);

impl Component for AnimatedTable {
    type Storage = DenseVecStorage<Self>;
}

//pub struct AnimatedSideTable;

//impl Component for AnimatedSideTable {
//type Storage = DenseVecStorage<Self>;
//}
