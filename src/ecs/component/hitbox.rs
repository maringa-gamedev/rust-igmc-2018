use amethyst::ecs::prelude::*;
use either::*;
use nalgebra::*;
use ncollide2d::shape::*;

pub struct Hitbox {
    pub shape: Either<Cuboid<f32>, Ball<f32>>,
    pub offset: Vector2<f32>,
}

impl Component for Hitbox {
    type Storage = DenseVecStorage<Self>;
}
