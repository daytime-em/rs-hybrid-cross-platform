use std::{hash::Hash, hash::Hasher, time::Duration};

pub struct PrimesResult {
  pub exec_time: Duration,
  pub primes: Vec<u64>,
  pub up_to: u64,
}

impl PrimesResult {
  pub fn primes_count(&self) -> usize {
    self.primes.len()
  }
}

pub trait FromPrimesResult {
    // Converts from a PrimesResult to something else
    fn from_primes_result(result: &PrimesResult) -> Self;
}

impl PartialEq for PrimesResult {
  fn eq(&self, other: &PrimesResult) -> bool {
    self.up_to == other.up_to
  }
}

impl Hash for PrimesResult {
  fn hash<H: Hasher>(&self, state: &mut H) {
    // Being unique on the value is enough
    self.up_to.hash(state)
  }
}
