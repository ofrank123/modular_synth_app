pub mod oscillators;

pub fn freq_from_midi(midi_val: f32) -> f32 {
    440.0 * 2.0_f32.powf((midi_val - 69.0) / 12.0)
}
