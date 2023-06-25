mod result;
mod sieves;

pub use result::PrimesResult;
pub use result::FromPrimesResult;

pub fn count_primes_simple(num: u64) -> PrimesResult {
  sieves::slow_sieve(&num)
}

pub fn count_primes_fast(num: u64) -> PrimesResult {
  sieves::bit_sieve(num)
}
