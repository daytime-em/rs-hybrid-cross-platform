use std::{hash::Hash, hash::Hasher};

/// A number and its factors
pub struct PrimesResult {
  pub count: u64,
  pub primes: Vec<u64>
}

pub trait FromPrimesResult {
    // Converts from a PrimesResult to something else
    fn from_primes_result(result: PrimesResult) -> Self;
}

impl PartialEq for PrimesResult {
  fn eq(&self, other: &PrimesResult) -> bool {
    self.count == other.count
  }
}

impl Hash for PrimesResult {
  fn hash<H: Hasher>(&self, state: &mut H) {
    // Being unique on the value is enough
    self.count.hash(state)
  }
}
