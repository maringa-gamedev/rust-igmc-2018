use super::animation::*;
use serde_derive::*;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct TextureSlice(pub String, pub f32, pub f32, pub f32, pub f32);

#[derive(Debug, Serialize, Deserialize)]
pub struct SpriteFolderDef {
    pub keys: Vec<String>,
    pub path: String,
    pub width: f32,
    pub height: f32,
    pub sprites: Vec<TextureSlice>,
    pub anims: HashMap<String, AnimationDef>,
}
