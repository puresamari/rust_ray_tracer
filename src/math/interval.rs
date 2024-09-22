use super::{max::max_f64, min::min_f64};

pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    pub fn empty() -> Self {
        Self {
            min: f64::INFINITY,
            max: f64::NEG_INFINITY,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.min > self.max
    }

    pub fn union(&self, other: &Self) -> Self {
        Self {
            min: min_f64(self.min, other.min),
            max: max_f64(self.max, other.max),
        }
    }

    pub fn intersection(&self, other: &Self) -> Self {
        Self {
            min: max_f64(self.min, other.min),
            max: min_f64(self.max, other.max),
        }
    }

    pub fn contains(&self, x: f64) -> bool {
        x >= self.min && x <= self.max
    }

    pub fn contains_interval(&self, other: &Self) -> bool {
        other.min >= self.min && other.max <= self.max
    }

    pub fn width(&self) -> f64 {
        self.max - self.min
    }

    pub fn center(&self) -> f64 {
        (self.min + self.max) / 2.0
    }

    pub fn scale(&self, factor: f64) -> Self {
        Self {
            min: self.min * factor,
            max: self.max * factor,
        }
    }

    pub fn translate(&self, offset: f64) -> Self {
        Self {
            min: self.min + offset,
            max: self.max + offset,
        }
    }

    pub fn lerp(&self, t: f64) -> f64 {
        self.min + (self.max - self.min) * t
    }

    pub fn surrounds(&self, x: f64) -> bool {
        x >= self.min && x <= self.max
    }

    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.min {
            self.min
        } else if x > self.max {
            self.max
        } else {
            x
        }
    }
}
