use std::f32::consts::PI;

pub fn sine_sample(phase: f32) -> f32 {
    phase.sin()
}

pub fn square_sample(phase: f32) -> f32 {
    if phase > PI {
        1.0
    } else {
        -1.0
    }
}

pub fn saw_sample(phase: f32) -> f32 {
    ((phase / PI) - 1.0) * -1.0
}

pub fn tri_sample(phase: f32) -> f32 {
    if phase < PI {
        (2.0 * phase / PI) - 1.0
    } else {
        ((-2.0 * phase) / PI) + 3.0
    }
}
