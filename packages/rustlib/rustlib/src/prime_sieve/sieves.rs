use bitvec::prelude::*;
use std::{
    collections::BTreeSet,
    time::Duration,
};

use super::{PrimesResult, Clock};

/// Computes primes using a sieve that tries to minimize mem use by storing marked
/// primes in a tree.
pub fn tree_sieve(up_to: u64, clock: &dyn Clock) -> PrimesResult {
    let mut sieve = TreeSieve::new(up_to);
    calculate(sieve.limit, &mut sieve.numbers, clock)
}

/// Computes primes using a bit buffer to store marked primes. This is the generally-
/// accepted way to write one of these
pub fn bit_sieve(up_to: u64, clock: &dyn Clock) -> PrimesResult {
    let mut sieve = BitArraySieve::new(up_to);
    calculate(sieve.limit, &mut sieve.numbers, clock)
}

/// Relies on trees to store marked numbers, very slow.
struct TreeSieve {
    limit: u64,
    numbers: BTreeSet<u64>,
}

/// PrimeSieve that uses a bit vector to represent the numbers. Marked numbers are 0
struct BitArraySieve {
    limit: u64,
    numbers: BitVec,
}

impl TreeSieve {
    fn new(up_to: u64) -> TreeSieve {
        TreeSieve {
            limit: up_to,
            numbers: BTreeSet::new(),
        }
    }
}

impl BitArraySieve {
    fn new(up_to: u64) -> BitArraySieve {
        let mut vec: BitVec<_, _> = BitVec::<usize, Lsb0>::new();
        // Pay the cost of (potentially) 1 usize in order to index from 1
        let len = (up_to as usize) + 1;
        vec.resize(len, false);
        BitArraySieve {
            limit: up_to,
            numbers: vec,
        }
    }
}

trait PrimeSieve {
    fn number_line<'a>(&self) -> &'a mut dyn NumberLine;
    // fn calculate(&mut self) -> PrimesResult;
}

trait NumberLine {
    fn mark_composite(&mut self, num: u64);
    fn is_marked_composite(&self, num: u64) -> bool;
    fn count_primes(&self, up_to: &u64) -> Vec<u64>;
}

impl NumberLine for BTreeSet<u64> {
    fn mark_composite(&mut self, num: u64) {
        self.insert(num);
    }

    fn is_marked_composite(&self, num: u64) -> bool {
        self.contains(&num)
    }

    fn count_primes(&self, up_to: &u64) -> Vec<u64> {
        let mut vec = Vec::<u64>::new();
        vec.push(2);
        for n in (3..*up_to).step_by(2) {
            if !self.contains(&n) {
                vec.push(n);
            }
        }
        vec.shrink_to_fit(); // For luck
        vec
    }
}

impl NumberLine for BitVec {
    fn mark_composite(&mut self, num: u64) {
        // Assumes that this space has been allocated
        self.set(num as usize, true)
    }

    fn is_marked_composite(&self, num: u64) -> bool {
        self[num as usize]
    }

    fn count_primes(&self, up_to: &u64) -> Vec<u64> {
        let mut out = Vec::<u64>::new();
        out.push(2);
        let mut num: usize = 3;
        let max = *up_to as usize;
        loop {
            if num >= max {
                break;
            }
            if !self[num] {
                out.push(num as u64);
            }
            num += 2;
        }
        out
    }
}

/// Simple prime sieve that doesn't care how its number line is represented.
/// Skips even numbers, only goes up to sqrt(up_to), 2 and 3 are freebies
fn calculate(up_to: u64, number_line: &mut dyn NumberLine, clock: &dyn Clock) -> PrimesResult {
    let started_at = clock.increasing_millis();
    // let number_line = sieve.number_line();

    match up_to {
        0..=1 => {
            // What primes?
            PrimesResult {
                primes: vec![],
                exec_time: calc_total_time(started_at, clock),
                up_to,
            }
        }
        2 => {
            // Freebie
            PrimesResult {
                primes: vec![2],
                exec_time: calc_total_time(started_at, clock),
                up_to,
            }
        }
        3 => {
            // Freebie
            PrimesResult {
                primes: vec![2, 3],
                exec_time: calc_total_time(started_at, clock),
                up_to,
            }
        }
        _ => {
            let mut num = 3;
            loop {
                if !number_line.is_marked_composite(num) {
                    // eliminate all multiples
                    mark_multiples(&num, &up_to, number_line);
                }

                if num * num >= up_to {
                    break;
                }
                num += 2;
            }

            let primes_vec = number_line.count_primes(&up_to);
            PrimesResult {
                exec_time: calc_total_time(started_at, clock),
                //exec_time: calc_total_time(started_at),
                primes: primes_vec,
                up_to,
            }
        }
    }
}

fn mark_multiples(num: &u64, up_to: &u64, into: &mut dyn NumberLine) {
    let mut n = *num;
    // loop from n to up_to, adding all multiples of up_to (until up_to) by multiplying by n
    loop {
        let multiple = num * n;
        if multiple > *up_to {
            break;
        }

        into.mark_composite(multiple);
        n += 2;
    }
}

fn calc_total_time(starting_time: Duration, clock: &dyn Clock) -> Duration {
  let now = clock.increasing_millis();
  now - starting_time
}

#[cfg(test)]
mod bit_sieve_tests {
    use crate::prime_sieve::{sieves::bit_sieve, SystemTimeClock};

    #[test]
    fn test_fourteen() {
        let num = 14;
        let expected_prime_ct = 6;

        let result = bit_sieve(num, &SystemTimeClock::new());

        assert_eq!(
            expected_prime_ct,
            result.primes_count(),
            "Expected {} primes, Got {}",
            expected_prime_ct,
            result.primes_count()
        );
    }

    #[test]
    fn test_two() {
        let num = 2;
        let expected_prime_ct = 1;

        let result = bit_sieve(num, &SystemTimeClock::new());

        assert_eq!(
            expected_prime_ct,
            result.primes_count(),
            "Expected {} primes, Got {}",
            expected_prime_ct,
            result.primes_count()
        );
    }

    #[test]
    fn test_three() {
        let num = 3;
        let expected_prime_ct = 2;

        let result = bit_sieve(num, &SystemTimeClock::new());

        assert_eq!(
            expected_prime_ct,
            result.primes_count(),
            "Expected {} primes, Got {}",
            expected_prime_ct,
            result.primes_count(),
        );
    }

    #[test]
    fn test_four() {
        let num = 4;
        let expected_prime_ct = 2;

        let result = bit_sieve(num, &SystemTimeClock::new());

        assert_eq!(
            expected_prime_ct,
            result.primes_count(),
            "Expected {} primes, Got {}",
            expected_prime_ct,
            result.primes_count()
        );
    }

    #[test]
    fn test_twenty_thousand() {
        let num = 20_000;
        let expected_prime_ct = 2262;

        let result = bit_sieve(num, &SystemTimeClock::new());

        assert_eq!(
            expected_prime_ct,
            result.primes_count(),
            "Expected {} primes, Got {}",
            expected_prime_ct,
            result.primes_count()
        );
    }

    #[test]
    fn test_two_thousand() {
        let num = 2_000;
        let expected_prime_ct = 303;

        let result = bit_sieve(num, &SystemTimeClock::new());

        assert_eq!(
            expected_prime_ct,
            result.primes_count(),
            "Expected {} primes, Got {}",
            expected_prime_ct,
            result.primes_count()
        );
    }

    #[test]
    fn test_two_hundred_k() {
        // kinda slow
        let num = 200_000;
        let expected_prime_ct = 17984;

        let result = bit_sieve(num, &SystemTimeClock::new());

        assert_eq!(
            expected_prime_ct,
            result.primes_count(),
            "Expected {} primes, Got {}",
            expected_prime_ct,
            result.primes_count()
        );
    }

    #[test]
    #[ignore = "slow"]
    fn test_twenty_m() {
        // kinda slow
        let num = 20_000_000;
        let expected_prime_ct = 1270607;

        let result = bit_sieve(num, &SystemTimeClock::new());

        assert_eq!(
            expected_prime_ct,
            result.primes_count(),
            "Expected {} primes, Got {}",
            expected_prime_ct,
            result.primes_count()
        );
    }
}

#[cfg(test)]
mod slow_sieve_tests {
    use crate::simple_sieve;

    #[test]
    fn test_fourteen() {
        let num = 14;
        let expected_prime_ct = 6;

        let result = simple_sieve(num);

        assert_eq!(
            expected_prime_ct,
            result.primes_count(),
            "Expected {} primes, Got {}",
            expected_prime_ct,
            result.primes_count()
        );
    }

    #[test]
    fn test_two() {
        let num = 2;
        let expected_prime_ct = 1;

        let result = simple_sieve(num);

        assert_eq!(
            expected_prime_ct,
            result.primes_count(),
            "Expected {} primes, Got {}",
            expected_prime_ct,
            result.primes_count()
        );
    }

    #[test]
    fn test_three() {
        let num = 3;
        let expected_prime_ct = 2;

        let result = simple_sieve(num);

        assert_eq!(
            expected_prime_ct,
            result.primes_count(),
            "Expected {} primes, Got {}",
            expected_prime_ct,
            result.primes_count()
        );
    }

    #[test]
    fn test_four() {
        let num = 4;
        let expected_prime_ct = 2;

        let result = simple_sieve(num);

        assert_eq!(
            expected_prime_ct,
            result.primes_count(),
            "Expected {} primes, Got {}",
            expected_prime_ct,
            result.primes_count()
        );
    }

    #[test]
    fn test_twenty_thousand() {
        let num = 20_000;
        let expected_prime_ct = 2262;

        let result = simple_sieve(num);

        assert_eq!(
            expected_prime_ct,
            result.primes_count(),
            "Expected {} primes, Got {}",
            expected_prime_ct,
            result.primes_count()
        );
    }

    #[test]
    fn test_two_thousand() {
        let num = 2_000;
        let expected_prime_ct = 303;

        let result = simple_sieve(num);

        assert_eq!(
            expected_prime_ct,
            result.primes_count(),
            "Expected {} primes, Got {}",
            expected_prime_ct,
            result.primes_count()
        );
    }

    #[test]
    #[ignore = "slow"]
    fn test_two_hundred_k() {
        // kinda slow
        let num = 200_000;
        let expected_prime_ct = 17984;

        let result = simple_sieve(num);

        assert_eq!(
            expected_prime_ct,
            result.primes_count(),
            "Expected {} primes, Got {}",
            expected_prime_ct,
            result.primes_count()
        );
    }
}
