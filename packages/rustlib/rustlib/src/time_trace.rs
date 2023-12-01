
pub trait TimeTrace {
  pub fn set(&mut self);
  pub fn duration_since(&self) -> Duration;
}

pub struct 