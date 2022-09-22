// Circular buffer for outputing
pub struct OutputBuffer {
  data: [f32; Self::LEN],
  read_head: usize,
  write_head: usize,
}

impl OutputBuffer {
  pub const LEN: usize = 1024;

  pub fn bytes_to_read(&self) -> usize {
    ((self.write_head - self.read_head) + Self::LEN) % Self::LEN
  }

  pub fn read(&mut self, n: usize) -> Vec<f32> {
    if self.bytes_to_read() > n {
      let slice = &self.data[self.read_head .. self.read_head + n].to_vec();
      self.read_head = (self.read_head + n) % Self::LEN;
      return slice.clone();
    }

    vec![]
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