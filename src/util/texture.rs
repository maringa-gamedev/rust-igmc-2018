use amethyst::renderer::SpriteSheetHandle;
use serde_derive::*;
use std::collections::HashMap;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TextureDefinition {
    pub path: String,
    pub width: f32,
    pub height: f32,
    pub sprites: Vec<(String, f32, f32, f32, f32)>,
}

#[derive(Debug)]
pub struct SpriteDefinition {
    pub handle: SpriteSheetHandle,
    pub sprites: HashMap<String, usize>,
}
