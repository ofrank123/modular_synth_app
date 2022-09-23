// Circular buffer for outputing
pub struct OutputBuffer {
  data: [f32; Self::LEN],
  read_head: usize,
  write_head: usize,
}

impl OutputBuffer {
  pub const LEN: usize = 1024;

  pub fn new() -> Self {
    OutputBuffer { data: [0.0; Self::LEN], read_head: 0, write_head: 0 }
  }

  pub fn samples_to_read(&self) -> usize {
    ((self.write_head - self.read_head) + Self::LEN) % Self::LEN
  }

  pub fn read(&mut self) -> f32 {
    if self.samples_to_read() != 0 {
      let sample = self.data[self.read_head];
      self.read_head = (self.read_head + 1) % Self::LEN;
      return sample;
    }

    0.0
  }

  pub fn write<'a, I>(&mut self, buffer: I)
  where
    I: Iterator<Item = &'a f32>
  {
    for &sample in buffer {
      self.data[self.write_head] = sample;
      self.write_head = (self.write_head + 1) % Self::LEN;
    }
  }
}