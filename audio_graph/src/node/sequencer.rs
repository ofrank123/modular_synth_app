pub struct SequencerNode {
    sample_rate: f32,
}

impl SequencerNode {
    const OUT_PORTS: [u32; 2] = [0, 1];

    pub fn new(sample_rate: f32) -> Self {
        SequencerNode { sample_rate }
    }
}
