mod prime_sieve;

pub mod examples;

pub use prime_sieve::Clock;
pub use prime_sieve::PrimesResult;
pub use prime_sieve::FromPrimesResult;

pub use examples::invoke_a_closure_cb;
use prime_sieve::SystemTimeClock;


// expose functionality by delegating
pub fn append_location_by_delegating(str: &str) -> String {
    let mut prepended = str.to_owned();
    prepended.insert_str(0, " by delegating,");
    examples::append_location(str)
}
/// fast prime sieve. Measures performance time with the given Clock object
pub fn fast_sieve_clock(up_to: u64, clock: &dyn Clock) -> PrimesResult {
  prime_sieve::count_primes_fast(up_to, clock)
}

/// fast prime sieve. Measure performance time with a SystemTimeClock
pub fn fast_sieve(up_to: u64) -> PrimesResult {
  prime_sieve::count_primes_simple(up_to, &SystemTimeClock::new())
}

/// fast prime sieve. Measure performance time with a SystemTimeClock
pub fn simple_sieve(up_to: u64) -> PrimesResult {
  prime_sieve::count_primes_simple(up_to, &SystemTimeClock::new())
}

/// slow prime sieve. Measures performance time with the given Clock object
pub fn simple_sieve_clock(up_to: u64, clock: &dyn Clock) -> PrimesResult {
  prime_sieve::count_primes_simple(up_to, clock)
}
