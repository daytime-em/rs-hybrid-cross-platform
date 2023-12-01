mod result;
mod sieves;

pub use result::PrimesResult;
pub use result::FromPrimesResult;

pub use sieves::tree_sieve as count_primes_simple;
pub use sieves::bit_sieve as count_primes_fast;
