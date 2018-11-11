pub trait InRange {
    fn in_range(&self, begin: Self, end: Self) -> bool;
}

impl InRange for f32 {
    fn in_range(&self, begin: f32, end: f32) -> bool {
        *self >= begin && *self < end
    }
}

impl InRange for f64 {
    fn in_range(&self, begin: f64, end: f64) -> bool {
        *self >= begin && *self < end
    }
}
