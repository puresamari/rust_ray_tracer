use super::interval::Interval;

pub fn random_f64() -> f64 {
    rand::random()
}

pub fn random_f64_in_interval(interval: &Interval) -> f64 {
    interval.min + (interval.max - interval.min) * random_f64()
}
