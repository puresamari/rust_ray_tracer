use serde::{Deserialize, Serialize};

use math::vec3::Vec3;

#[derive(Deserialize, Serialize, Clone, Copy)]
pub struct AnimationContext {
    pub frames_per_second: u32,
    pub shutter_speed: f64,
}

impl AnimationContext {
    pub fn time_at_frame(&self, frame: u32) -> f64 {
        (frame as f64) / (self.frames_per_second as f64)
    }
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[serde(tag = "type", content = "value")]
pub enum AnimatedValue {
    Static(f64),
    Sinusoidal {
        baseline: f64,
        frequency: f64,
        amplitude: f64,
        phase_shift: f64,
    },
}

impl AnimatedValue {
    pub fn value_at_time(&self, time: f64) -> f64 {
        match self {
            AnimatedValue::Static(value) => *value,
            AnimatedValue::Sinusoidal {
                baseline,
                frequency,
                amplitude,
                phase_shift,
            } => {
                baseline
                    + amplitude
                        * (2.0 * std::f64::consts::PI * frequency * time + phase_shift).sin()
            }
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct AnimatedVec3 {
    pub x: AnimatedValue,
    pub y: AnimatedValue,
    pub z: AnimatedValue,
}

impl AnimatedVec3 {
    pub fn value_at_time(&self, time: f64) -> Vec3 {
        Vec3::new(
            self.x.value_at_time(time),
            self.y.value_at_time(time),
            self.z.value_at_time(time),
        )
    }

    pub fn static_value(value: Vec3) -> Self {
        AnimatedVec3 {
            x: AnimatedValue::Static(value.x()),
            y: AnimatedValue::Static(value.y()),
            z: AnimatedValue::Static(value.z()),
        }
    }
}
