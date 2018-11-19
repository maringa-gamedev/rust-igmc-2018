use amethyst::{
    core::{
        cgmath::*,
        transform::{GlobalTransform, Transform},
    },
    ecs::prelude::*,
    renderer::SpriteRender,
};
use nk_data::*;
use nk_ecs::*;

pub fn generate_bg(world: &mut World) -> Vec<Entity> {
    let width_count = (VIEW_WIDTH / 16.0) as usize + 2;
    let height_count = (VIEW_HEIGHT / 16.0) as usize + 2;

    let bg_handle = { world.read_resource::<Handles>().bg_handle.clone() };
    (0..width_count)
        .map(|i| {
            (0..height_count)
                .map(|j| {
                    let mut transform = Transform::default();
                    transform.translation = Vector3::new(
                        8.0 + (i as isize - 1) as f32 * 16.0,
                        8.0 + (j as isize - 1) as f32 * 16.0,
                        -1020.0,
                    );
                    world
                        .create_entity()
                        .with(SpriteRender {
                            sprite_sheet: bg_handle.clone(),
                            sprite_number: 0,
                            flip_horizontal: false,
                            flip_vertical: false,
                        })
                        .with(Background {
                            from: Vector2::new(
                                8.0 + (i as isize - 1) as f32 * 16.0,
                                8.0 + (j as isize - 1) as f32 * 16.0,
                            ),
                            to: Vector2::new(8.0 + i as f32 * 16.0, 8.0 + j as f32 * 16.0),
                            timer: 0.0,
                        })
                        .with(transform)
                        .with(GlobalTransform::default())
                        .build()
                })
                .collect()
        })
        .fold(
            Vec::with_capacity(width_count * height_count),
            |mut acc, v: Vec<Entity>| {
                acc.extend(v);
                acc
            },
        )
}
