use amethyst::{
    core::bundle::{Result, SystemBundle},
    ecs::prelude::DispatcherBuilder,
};
use nk_ecs::*;

pub struct GameBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for GameBundle {
    fn build(self, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<()> {
        builder.add(BackgroundAnimationSystem, "xto_bg_anim", &[]);
        builder.add(InventoryRenderSystem, "xto_inventory_render", &[]);
        builder.add(ControllerSystem::new(), "xto_controller", &[]);
        builder.add(ControlSystem, "xto_control", &["xto_controller"]);
        builder.add(InputSystem, "xto_input", &["xto_control"]);
        builder.add(AnimationSystem, "xto_animation", &["xto_control"]);
        builder.add(AutotileSystem::default(), "xto_autotile", &["xto_control"]);
        builder.add(MovementSystem, "xto_movement", &["xto_input"]);
        builder.add(CollisionSystem, "xto_collision", &["xto_movement"]);
        builder.add(LayerSystem, "xto_layer", &["xto_collision"]);
        builder.add(InteractSystem, "xto_interact", &["xto_collision"]);
        builder.add(InteractionSystem, "xto_interaction", &["xto_interact"]);
        Ok(())
    }
}
