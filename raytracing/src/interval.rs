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

    pub fn new_covered(a: Interval, b: Interval) -> Self {
        let min = a.min.min(b.min);
        let max = a.max.max(b.max);
        Self { min, max }
    }

    pub fn min(&self) -> f64 {
        self.min
    }

    pub fn max(&self) -> f64 {
        self.max
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

    pub fn expand(&self, delta: f64) -> Self {
        let pad = delta / 2.0;
        Self {
            min: self.min - pad,
            max: self.max + pad,
        }
    }

    pub fn clamp_min(&mut self, x: f64) {
        if self.min < x {
            self.min = x;
        }
    }

    pub fn clamp_max(&mut self, x: f64) {
        if self.max > x {
            self.max = x;
        }
    }

    pub fn len(&self) -> f64 {
        self.max - self.min
    }
}

impl Default for Interval {
    fn default() -> Self {
        Self {
            min: f64::INFINITY,
            max: f64::NEG_INFINITY,
        }
    }
}
