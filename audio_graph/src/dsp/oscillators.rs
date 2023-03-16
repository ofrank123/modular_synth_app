use std::f32::consts::PI;

pub fn sine_sample(phase: f32) -> f32 {
    phase.sin()
}

// Generate a naive triangle wave sample from phase
pub fn naive_tri_sample(phase: f32) -> f32 {
    let val = -1.0 + (2.0 * phase / (2.0 * PI));
    2.0 * (val.abs() - 0.5)
}

// Generate a naive square wave sample from phase
pub fn naive_square_sample(phase: f32, pulse_width: f32) -> f32 {
    if phase < PI * 2.0 * pulse_width {
        1.0
    } else {
        -1.0
    }
}

// Generate a naive saw wave sample from phase
pub fn naive_saw_sample(phase: f32) -> f32 {
    (2.0 * phase / (2.0 * PI)) - 1.0
}

// Fix discontinuities in waveform
// https://www.martin-finke.de/articles/audio-plugins-018-polyblep-oscillator/
pub fn poly_blep(phase_inc: f32, mut t: f32) -> f32 {
    let dt = phase_inc / (2.0 * PI);

    // t-t^2/2+1/2
    if t < dt {
        t /= dt;
        return t + t - t * t - 1.0;
    } else if t > 1.0 - dt {
        t = (t - 1.0) / dt;
        return t * t + t + t + 1.0;
    } else {
        return 0.0;
    }
}
