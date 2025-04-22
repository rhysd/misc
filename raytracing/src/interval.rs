// Note: This type is very similar to `std::ops::Range<f64>` but it differs in terms of boundary comparisons.
#[derive(Clone, Copy)]
pub struct Interval {
    min: f64,
    max: f64,
}

impl Interval {
    pub const fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    pub const EMPTY: Self = Self::new(f64::INFINITY, f64::NEG_INFINITY);
    pub const UNIVERSE: Self = Self::new(f64::NEG_INFINITY, f64::INFINITY);
}
