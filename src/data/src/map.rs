use amethyst::core::cgmath::*;
use serde_derive::*;

#[derive(Debug, Serialize, Deserialize)]
pub enum TableOrientation {
    VerticalLeft,
    VerticalRight,
    HorizontalTop,
    HorizontalBottom,
}

impl Default for TableOrientation {
    fn default() -> Self {
        TableOrientation::HorizontalBottom
    }
}

impl TableOrientation {
    pub fn flip_vertical(&self) -> bool {
        match self {
            TableOrientation::VerticalLeft => true,
            TableOrientation::VerticalRight => false,
            TableOrientation::HorizontalTop => true,
            TableOrientation::HorizontalBottom => false,
        }
    }

    pub fn make_dim(&self, w: f32, h: f32) -> (f32, f32) {
        match self {
            TableOrientation::VerticalLeft => (h, w),
            TableOrientation::VerticalRight => (h, w),
            TableOrientation::HorizontalTop => (w, h),
            TableOrientation::HorizontalBottom => (w, h),
        }
    }

    pub fn make_rot(&self) -> Deg<f32> {
        match self {
            TableOrientation::VerticalLeft => Deg(90.0),
            TableOrientation::VerticalRight => Deg(90.0),
            TableOrientation::HorizontalTop => Deg(0.0),
            TableOrientation::HorizontalBottom => Deg(0.0),
        }
    }

    pub fn make_scale(&self) -> f32 {
        match self {
            TableOrientation::VerticalLeft => 1.0,
            TableOrientation::VerticalRight => 1.0,
            TableOrientation::HorizontalTop => 2.0,
            TableOrientation::HorizontalBottom => 2.0,
        }
    }

    pub fn hitbox_height_multiplier(&self) -> f32 {
        match self {
            TableOrientation::VerticalLeft => 0.75,
            TableOrientation::VerticalRight => 0.75,
            TableOrientation::HorizontalTop => 0.5,
            TableOrientation::HorizontalBottom => 0.5,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Priority(pub usize);

#[derive(Debug, Serialize, Deserialize)]
pub enum TableType {
    Flavor(Priority),
    Preparation,
    Topping,
    Delivery,
    Empty,
}

impl Default for TableType {
    fn default() -> Self {
        TableType::Empty
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct MapDefinition {
    pub tables: Vec<(f32, f32, TableType, TableOrientation)>,
    pub spawns: Vec<(f32, f32)>,
}
