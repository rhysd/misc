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

    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    pub fn clamp(&self, x: f64) -> f64 {
        // Note: `f64::clamp` is not available because it panics when `min` > `max` in debug build.
        if x < self.min {
            self.min
        } else if self.max < x {
            self.max
        } else {
            x
        }
    }
}
