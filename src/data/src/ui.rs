use crate::common::*;
use serde_derive::*;

#[derive(Debug, Serialize, Deserialize)]
pub enum UiSide {
    Top,
    Left,
    Bottom,
    Right,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum UiCorner {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum UiAction {
    Back,
    Confirm,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum UiSize {
    Small,
    Big,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum UiIndex {
    Previous,
    Current,
    Next,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum UiDirection {
    Previous,
    Next,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum UiSprite {
    FullCorner(UiCorner),
    HalfCorner(UiCorner),
    CutCorner(UiCorner),
    ConnectCorner(UiCorner),
    FullSide(UiSide),
    HalfSide(UiSide),
    Center,
    Button(UiAction),
    ToggleFlavor(usize, usize, FlavorIndex),
    TogglePreparation(usize, usize, PreparationIndex),
    ToggleTopping(usize, usize, ToppingIndex),
    MapPreview(UiIndex),
    Arrow(UiDirection),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UiElement(pub f32, pub f32, pub UiSprite);

#[derive(Debug, Serialize, Deserialize)]
pub struct UiDefinition {
    pub sprites: Vec<UiElement>,
}
