mod prime_sieve;

pub mod examples;

pub use prime_sieve::PrimesResult;
pub use prime_sieve::FromPrimesResult;

pub use examples::invoke_a_closure_cb;


// expose functionality by delegating
pub fn append_location_by_delegating(str: &str) -> String {
    let mut prepended = str.to_owned();
    prepended.insert_str(0, " by delegating,");
    examples::append_location(str)
}

/// simple prime sieve
pub fn simple_sieve(up_to: u64) -> PrimesResult {
  prime_sieve::count_primes_simple(up_to)
}

pub fn fast_sieve(up_to: u64) -> PrimesResult {
  prime_sieve::count_primes_fast(up_to)
}
