use std::mem::ManuallyDrop;

use rustlib::FromPrimesResult;

#[repr(C)]
pub struct FoundPrimes {
    pub prime_count: u64,
    pub primes: *mut u64,
}

#[no_mangle]
pub extern "C" fn free_found_primes(found_primes: FoundPrimes) {
    // free the vector of primes.
    unsafe {
        let vec = Vec::from_raw_parts(
            found_primes.primes,
            found_primes.prime_count.try_into().unwrap(),
            found_primes.prime_count.try_into().unwrap(),
        );
        drop(vec);
    }
}

impl FromPrimesResult for FoundPrimes {
    fn from_primes_result(result: rustlib::PrimesResult) -> Self {
        let primes_vec = result.primes;
        let mut vec_box = ManuallyDrop::new(primes_vec.into_boxed_slice());
        FoundPrimes {
            prime_count: result.count,
            primes: vec_box.as_mut_ptr(),
        }
    }
}
